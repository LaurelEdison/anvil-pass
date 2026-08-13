use std::{
    path::PathBuf,
    sync::{Mutex, MutexGuard},
};

use anvil_core::vault::Vault;

use thiserror::Error;

use crate::state::AppError::PoisonedState;

#[derive(Debug, Error)]
pub enum AppError {
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
    pub fn get_vault(&self) -> Result<MutexGuard<'_, Option<Vault>>, AppError> {
        let guard = self.vault.lock().map_err(|e| PoisonedState(e.to_string()));
        guard
    }
}
