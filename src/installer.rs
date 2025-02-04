use tokio::fs;
use tokio::process::Command;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Install failed")]
    ExecutionFailed,
}

pub async fn install_driver(path: &Path) -> Result<(), InstallError> {
    let temp_dir = tempfile::tempdir()?;
    let dest_path = temp_dir.path().join("driver.bin");
    fs::copy(path, &dest_path).await?;

    let status = Command::new("pnputil")
        .args(&["/add-driver", dest_path.to_str().unwrap(), "/install"])
        .status()
        .await?;

    if status.success() {
        Ok(())
    } else {
        Err(InstallError::ExecutionFailed)
    }
}