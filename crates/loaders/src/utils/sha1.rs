use std::path::Path;
use tokio::fs;
use sha1::{Sha1, Digest};


#[derive(Debug, thiserror::Error)]
pub enum Sha1Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SHA1 mismatch: expected {expected}, got {actual}")]
    Mismatch { expected: String, actual: String },
}

pub type Sha1Result<T> = Result<T, Sha1Error>;

/// Vérifie le SHA1 d'un fichier de manière optimisée
pub async fn verify_file_sha1(path: &Path, expected_sha1: &str) -> Sha1Result<bool> {
    // Lecture optimisée
    let content = fs::read(path).await?;

    // Calcul SHA1
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let calculated_sha1 = hex::encode(hasher.finalize());

    Ok(calculated_sha1.eq_ignore_ascii_case(expected_sha1))
}

/// Version optimisée qui lit en streaming pour gros fichiers
pub async fn verify_file_sha1_streaming(path: &Path, expected_sha1: &str) -> Sha1Result<bool> {
    use tokio::io::AsyncReadExt;

    let mut file = fs::File::open(path).await?;
    let mut hasher = Sha1::new();
    let mut buffer = vec![0u8; 8192]; // 8KB buffer

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let calculated_sha1 = hex::encode(hasher.finalize());
    Ok(calculated_sha1.eq_ignore_ascii_case(expected_sha1))
}