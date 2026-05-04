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

/// Download file using HTTP_CLIENT without any progress tracking
pub async fn download_file_untracked(url: &str, path: impl AsRef<Path>) -> DownloadResult<()> {
    let path = path.as_ref().to_owned();
    let mut last_error = None;

    for candidate in build_fallback_urls(url) {
        match HTTP_CLIENT.get(&candidate).send().await {
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

pub async fn download_file<F>(url: &str, on_progress: F) -> DownloadResult<Vec<u8>>
where
    F: Fn(u64, u64),
{
    trace_debug!("Downloading file {:?}", url);

    let mut last_error = None;

    for candidate in build_fallback_urls(url) {
        let mut response = match HTTP_CLIENT
            .get(candidate.trim())
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

        let max_len = response.content_length().unwrap_or(0);
        let mut output = Vec::with_capacity(max_len as usize);
        let mut curr_len = 0;

        on_progress(0, max_len);

        trace_debug!("Reading data from response chunk...");
        while let Some(data) = response.chunk().await? {
            output.extend_from_slice(&data);
            curr_len += data.len();
            on_progress(curr_len as u64, max_len);
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
