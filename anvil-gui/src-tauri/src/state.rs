use std::{
    path::PathBuf,
    sync::{Mutex, MutexGuard},
};

use anvil_core::vault::{DatabaseProcessingError, Vault};

use thiserror::Error;

use crate::state::AppError::PoisonedState;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to create entry: {0}")]
    GroupDelete(DatabaseProcessingError),
    #[error("Failed to create entry: {0}")]
    GroupCreate(DatabaseProcessingError),
    #[error("Failed to create entry: {0}")]
    GroupUpdate(DatabaseProcessingError),

    #[error("Failed to create entry: {0}")]
    EntryCreate(DatabaseProcessingError),
    #[error("Failed to delete entry: {0}")]
    EntryDelete(DatabaseProcessingError),
    #[error("Failed to update entry: {0}")]
    EntryUpdate(DatabaseProcessingError),

    #[error("Failed to create vault: {0}")]
    VaultCreate(DatabaseProcessingError),
    #[error("Failed to save vault: {0}")]
    VaultSave(DatabaseProcessingError),
    #[error("Failed to open vault: {0}")]
    VaultOpen(DatabaseProcessingError),
    #[error("Vault locked")]
    VaultLocked,
    #[error("Vault not defined")]
    VaultNone,
    #[error("State poisoned: {0}")]
    PoisonedState(String),
    #[error("Vault path not defined")]
    NonePath,
}

pub struct AppState {
    pub vault: Mutex<Option<Vault>>,
    pub vault_path: Mutex<Option<PathBuf>>,
    pub master_password: Mutex<String>,
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
    pub fn get_master_password(&self) -> Result<MutexGuard<'_, String>, AppError> {
        let guard = self
            .master_password
            .lock()
            .map_err(|e| PoisonedState(e.to_string()));
        guard
    }
}
