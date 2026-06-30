pub mod backup;
pub mod database;
pub mod entries;
pub mod groups;
pub mod search;
pub mod traversal;

use std::path::PathBuf;

use keepass::{
    Database,
    db::{DestinationGroupNotFoundError, MoveGroupError},
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DatabaseProcessingError {
    #[error("failed to find entry with id: {0}")]
    FailedToFindEntry(Uuid),

    #[error("failed to find entry with title: {0}")]
    FailedToFindEntryByTitle(String),
    #[error("Failed to get title from entry {0}")]
    FailedToGetTitleFromEntry(Uuid),

    #[error("failed to move entry with id{src_id} to group with id{dst_id}: error: {err}")]
    FailedToMoveEntry {
        src_id: Uuid,
        dst_id: Uuid,
        err: DestinationGroupNotFoundError,
    },

    #[error("failed to move group with id{src_id} to group with id{dst_id}: error: {err}")]
    FailedToMoveGroup {
        src_id: Uuid,
        dst_id: Uuid,
        err: MoveGroupError,
    },

    #[error("failed to find group with id: {0}")]
    FailedToFindGroup(Uuid),
    #[error("failed to find entry with name: {0}")]
    FailedToFindGroupByName(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Save error: {0}")]
    Save(#[from] keepass::error::DatabaseSaveError),

    #[error("Open error: {0}")]
    Open(#[from] keepass::db::DatabaseOpenError),

    #[error("backup failed due to unsaved changes")]
    DirtyDatabase,
    #[error("vault file not found")]
    VaultFileMissing,
    #[error("invalid regex")]
    InvalidRegex,
}

#[derive(Debug)]
pub struct Vault {
    pub database: Database,
    pub path: PathBuf,
    pub dirty: bool,
}
impl Vault {}
