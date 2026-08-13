use std::{
    path::PathBuf,
    sync::{Mutex, MutexGuard},
};

use anvil_core::vault::{DatabaseProcessingError, Vault};

use thiserror::Error;

use crate::state::AppError::PoisonedState;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to create vault: {0}")]
    CreateVault(DatabaseProcessingError),
    #[error("Failed to open vault: {0}")]
    OpenVault(DatabaseProcessingError),
    #[error("Vault locked")]
    VaultLocked,
    #[error("State poisoned: {0}")]
    PoisonedState(String),
    #[error("Vault not defined")]
    NoneVault,
    #[error("Vault path not defined")]
    NonePath,
}

pub struct AppState {
    pub vault: Mutex<Option<Vault>>,
    pub vault_path: Mutex<Option<PathBuf>>,
}

impl AppState {
    pub fn set_vault(&self, vault: Vault, path: PathBuf) -> Result<(), AppError> {
        let mut vault_lock = self
            .vault
            .lock()
            .map_err(|e| AppError::PoisonedState(e.to_string()))?;
        let mut path_lock = self
            .vault_path
            .lock()
            .map_err(|e| AppError::PoisonedState(e.to_string()))?;

        *vault_lock = Some(vault);
        *path_lock = Some(path);

        Ok(())
    }
    pub fn get_vault(&self) -> Result<MutexGuard<'_, Option<Vault>>, AppError> {
        let guard = self.vault.lock().map_err(|e| PoisonedState(e.to_string()));
        guard
    }
}
