use anvil_core::vault;

use crate::dto::{EntryDto, GroupDto};

pub struct AppState {
    entries: Vec<EntryDto>,
    groups: Vec<GroupDto>,
    vault: Option<vault::Vault>,
}
impl AppState {
    pub fn init() {}
}
