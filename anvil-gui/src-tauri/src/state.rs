use std::{path::PathBuf, sync::Mutex};

use anvil_core::vault::Vault;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Failed to create entry: {0}")]
    GroupDelete(String),
    #[error("Failed to create entry: {0}")]
    GroupCreate(String),
    #[error("Failed to create entry: {0}")]
    GroupUpdate(String),

    #[error("Failed to complete task: {0}")]
    Task(String),

    #[error("Failed to create entry: {0}")]
    EntryCreate(String),
    #[error("Failed to delete entry: {0}")]
    EntryDelete(String),
    #[error("Failed to update entry: {0}")]
    EntryUpdate(String),

    #[error("Failed to create vault: {0}")]
    VaultCreate(String),
    #[error("Failed to save vault: {0}")]
    VaultSave(String),
    #[error("Failed to open vault: {0}")]
    VaultOpen(String),
    #[error("Vault locked")]
    StateLocked(String),
    #[error("Vault not defined")]
    VaultNone,
    #[error("State poisoned: {0}")]
    PoisonedState(String),
    #[error("Vault path not defined")]
    NonePath,
}

pub struct AppState {
    pub vault: Option<Vault>,
    pub vault_path: Option<PathBuf>,
    pub master_password: String,
}

impl AppState {
    pub fn new() -> Mutex<AppState> {
        Mutex::new(AppState {
            vault: (None),
            vault_path: (None),
            master_password: (String::new()),
        })
    }
    pub fn reset(&mut self) -> Result<(), AppError> {
        self.vault = None;
        self.vault_path = None;
        self.master_password = String::new();
        Ok(())
    }
    pub fn set_vault(
        &mut self,
        vault: Vault,
        path: PathBuf,
        master_password: String,
    ) -> Result<(), AppError> {
        self.vault = Some(vault);
        self.vault_path = Some(path);
        self.master_password = master_password;
        Ok(())
    }
}
