/*
 * This file is part of LiquidLauncher (https://github.com/CCBlueX/LiquidLauncher)
 *
 * Copyright (c) 2015 - 2024 CCBlueX
 *
 * LiquidLauncher is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * LiquidLauncher is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with LiquidLauncher. If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::Path;

use crate::errors::{DownloadResult, DownloadError};
use crate::{trace_debug};
use tokio::fs;
use crate::hosts::{HTTP_CLIENT, RAW_HTTP_CLIENT, CLOUDFLARE_PROXY_CLIENT, build_fallback_urls};
use reqwest::header::{ACCEPT_ENCODING, CONTENT_ENCODING};

/// Downloads `url` to `path` without progress reporting.
///
/// Used for small one-shot fetches where streaming and progress callbacks
/// would be overkill (e.g. mod-loader installer JARs, single manifests).
///
/// Retry strategy (in order):
/// 1. Try all fallback URLs with the standard HTTP client
/// 2. If decode errors occur, retry with raw (no decompression) client
/// 3. If all above fails, retry all fallback URLs using Cloudflare 1.1.1.1 proxy
/// 4. If proxy+decode error, retry with raw client through proxy
pub async fn download_file_untracked(url: &str, path: impl AsRef<Path>) -> DownloadResult<()> {
    let path = path.as_ref().to_owned();
    let mut last_error = None;
    let urls = build_fallback_urls(url);

    // First attempt: standard clients
    for candidate in &urls {
        match download_untracked_once(&HTTP_CLIENT, &candidate, &path).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                let should_retry_raw = is_decode_error(&e);
                last_error = Some(e);

                if should_retry_raw {
                    trace_debug!(
                        url = %candidate,
                        "Response decode failed; retrying download without automatic decompression"
                    );

                    match download_untracked_once(&RAW_HTTP_CLIENT, &candidate, &path).await {
                        Ok(_) => return Ok(()),
                        Err(raw_err) => {
                            last_error = Some(raw_err);
                        }
                    }
                }
            }
        }
    }

    // Second attempt: retry all URLs using Cloudflare proxy as last resort
    trace_debug!(
        url = %url,
        "All standard attempts failed; retrying with Cloudflare 1.1.1.1 proxy"
    );
    for candidate in &urls {
        match download_untracked_once(&CLOUDFLARE_PROXY_CLIENT, &candidate, &path).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                let should_retry_raw = is_decode_error(&e);
                last_error = Some(e);

                if should_retry_raw {
                    trace_debug!(
                        url = %candidate,
                        "Proxy response decode failed; retrying through proxy without automatic decompression"
                    );

                    match download_untracked_once(&RAW_HTTP_CLIENT, &candidate, &path).await {
                        Ok(_) => return Ok(()),
                        Err(raw_err) => {
                            last_error = Some(raw_err);
                        }
                    }
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        DownloadError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No candidates available for download (all standard and proxy attempts exhausted)",
        ))
    }))
}

fn is_decode_error(err: &DownloadError) -> bool {
    match err {
        DownloadError::Http(http_err) => http_err.is_decode(),
        _ => false,
    }
}

async fn download_untracked_once(
    client: &reqwest::Client,
    url: &str,
    path: &Path,
) -> DownloadResult<()> {
    let response = client
        .get(url)
        .header(ACCEPT_ENCODING, "identity")
        .send()
        .await?
        .error_for_status()?;

    let content = response.bytes().await?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::write(path, content).await?;
    Ok(())
}

/// Downloads `url` into a `Vec<u8>`, invoking `on_progress(current, total)`
/// after each chunk.
///
/// `total` is taken from `Content-Length` and is `0` when the server does
/// not announce one. The function returns the complete body once the
/// response stream ends.
///
/// Retry strategy (in order):
/// 1. Try all fallback URLs with the standard HTTP client
/// 2. If decode errors occur, retry with raw (no decompression) client
/// 3. If all above fails, retry all fallback URLs using Cloudflare 1.1.1.1 proxy
/// 4. If proxy+decode error, retry with raw client through proxy
pub async fn download_file<F>(url: &str, on_progress: F) -> DownloadResult<Vec<u8>>
where
    F: Fn(u64, u64),
{
    trace_debug!("Downloading file {:?}", url);

    let mut last_error = None;
    let urls = build_fallback_urls(url);

    // First attempt: standard clients
    for candidate in &urls {
        let mut response = match download_streaming_once(&HTTP_CLIENT, candidate.trim()).await {
            Ok(response) => response,
            Err(e) => {
                if is_decode_error(&e) {
                    trace_debug!(
                        url = %candidate,
                        "Response decode failed; retrying streaming download without automatic decompression"
                    );
                    match download_streaming_once(&RAW_HTTP_CLIENT, candidate.trim()).await {
                        Ok(response) => response,
                        Err(raw_err) => {
                            last_error = Some(raw_err);
                            continue;
                        }
                    }
                } else {
                    last_error = Some(e);
                    continue;
                }
            }
        };

        trace_debug!("Response received from url");

        let encoding = response
            .headers()
            .get(CONTENT_ENCODING)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("identity");
        let is_identity = encoding.eq_ignore_ascii_case("identity");
        let max_len = if is_identity {
            response.content_length().unwrap_or(0)
        } else {
            0
        };
        let mut output = Vec::with_capacity(max_len as usize);
        let mut curr_len = 0;

        on_progress(0, max_len);

        trace_debug!("Reading data from response chunk...");
        while let Some(data) = response.chunk().await? {
            output.extend_from_slice(&data);
            curr_len += data.len();
            if max_len > 0 {
                let capped = (curr_len as u64).min(max_len);
                on_progress(capped, max_len);
            } else {
                on_progress(curr_len as u64, max_len);
            }
        }

        trace_debug!("Downloaded file");
        return Ok(output);
    }

    // Second attempt: retry all URLs using Cloudflare proxy as last resort
    trace_debug!(
        url = %url,
        "All standard streaming attempts failed; retrying with Cloudflare 1.1.1.1 proxy"
    );
    for candidate in &urls {
        let mut response = match download_streaming_once(&CLOUDFLARE_PROXY_CLIENT, candidate.trim()).await {
            Ok(response) => response,
            Err(e) => {
                if is_decode_error(&e) {
                    trace_debug!(
                        url = %candidate,
                        "Proxy response decode failed; retrying through proxy without automatic decompression"
                    );
                    match download_streaming_once(&RAW_HTTP_CLIENT, candidate.trim()).await {
                        Ok(response) => response,
                        Err(raw_err) => {
                            last_error = Some(raw_err);
                            continue;
                        }
                    }
                } else {
                    last_error = Some(e);
                    continue;
                }
            }
        };

        trace_debug!("Response received from proxy url");

        let encoding = response
            .headers()
            .get(CONTENT_ENCODING)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("identity");
        let is_identity = encoding.eq_ignore_ascii_case("identity");
        let max_len = if is_identity {
            response.content_length().unwrap_or(0)
        } else {
            0
        };
        let mut output = Vec::with_capacity(max_len as usize);
        let mut curr_len = 0;

        on_progress(0, max_len);

        trace_debug!("Reading data from proxy response chunk...");
        while let Some(data) = response.chunk().await? {
            output.extend_from_slice(&data);
            curr_len += data.len();
            if max_len > 0 {
                let capped = (curr_len as u64).min(max_len);
                on_progress(capped, max_len);
            } else {
                on_progress(curr_len as u64, max_len);
            }
        }

        trace_debug!("Downloaded file via proxy");
        return Ok(output);
    }

    Err(last_error.unwrap_or_else(|| {
        DownloadError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No candidates available for download (all standard and proxy attempts exhausted)",
        ))
    }))
}

async fn download_streaming_once(
    client: &reqwest::Client,
    url: &str,
) -> DownloadResult<reqwest::Response> {
    Ok(client
        .get(url)
        .header(ACCEPT_ENCODING, "identity")
        .send()
        .await?
        .error_for_status()?)
}
