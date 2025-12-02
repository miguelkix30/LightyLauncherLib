use thiserror::Error;

/// Erreurs possibles lors de l'installation
#[derive(Debug, Error)]
pub enum InstallerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("SHA1 verification failed: {0}")]
    Sha1(#[from] crate::minecraft::utils::sha1::Sha1Error),

    #[error("Loader not supported: {0}")]
    UnsupportedLoader(String),

    #[error("Invalid metadata: expected VersionBuilder")]
    InvalidMetadata,

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
}

pub type InstallerResult<T> = std::result::Result<T, InstallerError>;
