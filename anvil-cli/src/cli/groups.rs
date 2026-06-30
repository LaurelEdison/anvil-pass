use std::{path::PathBuf, str::FromStr};

use anvil_core::{
    GroupId,
    vault::{database::open_vault, groups::NewGroup},
};
use uuid::Uuid;
// TODO replace all unwrap with actual error handling

pub fn handle_add_group(
    vault: PathBuf,
    master_password: String,
    name: String,
    tags: Option<Vec<String>>,
    notes: Option<String>,
    parent: Option<String>,
) {
    let mut vault = open_vault(master_password.as_str(), vault).unwrap();
    let mut new_group = NewGroup::new(name);

    if let Some(tags) = tags {
        new_group = new_group.with_tags(tags);
    }
    if let Some(notes) = notes {
        new_group = new_group.with_notes(notes);
    }

    if let Some(group) = parent {
        let group_id = Uuid::from_str(group.as_str()).unwrap();
        let group = GroupId::from_uuid(group_id);
        new_group = new_group.with_parent_group(group);
    }
    vault.add_group(new_group);
    vault.save(master_password.as_str());
}
pub fn handle_edit_group() {}
pub fn handle_remove_group(vault: PathBuf, master_password: String, id: String) {
    let mut vault = open_vault(master_password.as_str(), vault).unwrap();
    let entry_id = Uuid::from_str(id.as_str()).unwrap();
    vault.delete_entry(entry_id);
    vault.save(master_password.as_str());
}
