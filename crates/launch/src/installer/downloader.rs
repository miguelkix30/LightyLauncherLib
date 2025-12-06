// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! File download utilities with retry logic and concurrency control

use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::Semaphore;
use tracing::{error, warn};
use futures::future::try_join_all;
use futures::StreamExt;
use lighty_core::hosts::HTTP_CLIENT as CLIENT;
use lighty_core::mkdir;
use crate::errors::InstallerResult;
use crate::errors::InstallerError;
//TODO: Make this configurable with a instance
/// Maximum number of concurrent downloads to prevent socket exhaustion
pub const MAX_CONCURRENT_DOWNLOADS: usize = 50;

/// Downloads small files (loaded entirely in memory)
pub async fn download_small_file(url: String, dest: PathBuf) -> InstallerResult<()> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_DELAY_MS: u64 = 20;

    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match download_small_file_once(&url, &dest).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                if attempt < MAX_RETRIES {
                    let delay = INITIAL_DELAY_MS * 2u64.pow(attempt - 1);
                    warn!(
                        "[Retry {}/{}] Failed to download {}: {}. Retrying in {}ms...",
                        attempt, MAX_RETRIES, url, e, delay
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap())
}

async fn download_small_file_once(url: &str, dest: &PathBuf) -> InstallerResult<()> {
    let bytes = CLIENT.get(url).send().await?.bytes().await?;

    if let Some(parent) = dest.parent() {
        mkdir!(parent);
    }

    fs::write(dest, bytes).await?;
    Ok(())
}

/// Downloads large files with streaming (memory efficient)
pub async fn download_large_file(url: String, dest: PathBuf) -> InstallerResult<()> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_DELAY_MS: u64 = 20;

    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match download_large_file_once(&url, &dest).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                if attempt < MAX_RETRIES {
                    let delay = INITIAL_DELAY_MS * 2u64.pow(attempt - 1);
                    warn!(
                        "[Retry {}/{}] Failed to download {}: {}. Retrying in {}ms...",
                        attempt, MAX_RETRIES, url, e, delay
                    );
                    let _ = fs::remove_file(&dest).await;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap())
}

async fn download_large_file_once(url: &str, dest: &PathBuf) -> InstallerResult<()> {
    let response = CLIENT.get(url).send().await?;

    if !response.status().is_success() {
        return Err(InstallerError::DownloadFailed(format!(
            "HTTP {} for {}",
            response.status(),
            url
        )));
    }

    if let Some(parent) = dest.parent() {
        mkdir!(parent);
    }

    let file = fs::File::create(dest).await?;
    let mut writer = BufWriter::with_capacity(256 * 1024, file);
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        writer.write_all(&chunk).await?;
    }

    writer.flush().await?;
    Ok(())
}

/// Downloads multiple large files with concurrency limit
pub async fn download_with_concurrency_limit(tasks: Vec<(String, PathBuf)>) -> InstallerResult<()> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_DOWNLOADS));
    let futures: Vec<_> = tasks
        .into_iter()
        .map(|(url, dest)| {
            let sem = semaphore.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();
                download_large_file(url, dest).await
            }
        })
        .collect();

    try_join_all(futures).await?;
    Ok(())
}

/// Downloads multiple small files with concurrency limit
pub async fn download_small_with_concurrency_limit(tasks: Vec<(String, PathBuf)>) -> InstallerResult<()> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_DOWNLOADS));
    let futures: Vec<_> = tasks
        .into_iter()
        .map(|(url, dest)| {
            let sem = semaphore.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();
                download_small_file(url, dest).await
            }
        })
        .collect();

    try_join_all(futures).await?;
    Ok(())
}
