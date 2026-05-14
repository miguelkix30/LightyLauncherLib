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
use crate::hosts::{HTTP_CLIENT, build_fallback_urls};
use reqwest::header::{ACCEPT_ENCODING, CONTENT_ENCODING};

/// Downloads `url` to `path` without progress reporting.
///
/// Used for small one-shot fetches where streaming and progress callbacks
/// would be overkill (e.g. mod-loader installer JARs, single manifests).
pub async fn download_file_untracked(url: &str, path: impl AsRef<Path>) -> DownloadResult<()> {
    let path = path.as_ref().to_owned();
    let mut last_error = None;

    for candidate in build_fallback_urls(url) {
        match HTTP_CLIENT
            .get(&candidate)
            .header(ACCEPT_ENCODING, "identity")
            .send()
            .await {
            Ok(response) => match response.error_for_status() {
                Ok(response) => {
                    let content = response.bytes().await?;
                    fs::write(&path, content).await?;
                    return Ok(());
                }
                Err(e) => {
                    last_error = Some(e);
                }
            },
            Err(e) => {
                last_error = Some(e);
            }
        }
    }

    Err(last_error
        .map(DownloadError::Http)
        .unwrap_or_else(|| {
            DownloadError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No candidates available for download",
            ))
        }))
}

/// Downloads `url` into a `Vec<u8>`, invoking `on_progress(current, total)`
/// after each chunk.
///
/// `total` is taken from `Content-Length` and is `0` when the server does
/// not announce one. The function returns the complete body once the
/// response stream ends.
pub async fn download_file<F>(url: &str, on_progress: F) -> DownloadResult<Vec<u8>>
where
    F: Fn(u64, u64),
{
    trace_debug!("Downloading file {:?}", url);

    let mut last_error = None;

    for candidate in build_fallback_urls(url) {
        let mut response = match HTTP_CLIENT
            .get(candidate.trim())
            .header(ACCEPT_ENCODING, "identity")
            .send()
            .await
            .and_then(|resp| resp.error_for_status())
        {
            Ok(response) => response,
            Err(e) => {
                last_error = Some(e);
                continue;
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

    Err(last_error
        .map(DownloadError::Http)
        .unwrap_or_else(|| {
            DownloadError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No candidates available for download",
            ))
        }))
}
