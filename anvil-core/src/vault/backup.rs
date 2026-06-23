use std::path::PathBuf;

use crate::vault::{DatabaseProcessingError, Vault};

impl Vault {
    pub fn create_backup(&self, backup_path: PathBuf) -> Result<(), DatabaseProcessingError> {
        if self.dirty {
            return Err(DatabaseProcessingError::DirtyDatabase);
        }
        if !self.path.exists() {
            return Err(DatabaseProcessingError::VaultFileMissing);
        }
        std::fs::copy(&self.path, backup_path).map_err(DatabaseProcessingError::Io)?;
        Ok(())
    }
}
