use std::{path::PathBuf, str::FromStr};

// TODO replace all unwrap with actual error handling
use anvil_core::{
    GroupId,
    vault::{
        database::open_vault,
        entries::{NewEntry, UpdateEntry},
    },
};
use uuid::Uuid;

pub fn handle_add_entry(
    vault: PathBuf,
    master_password: String,
    password: String,
    title: String,
    username: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    totp: Option<String>,
    group: Option<String>,
) {
    let mut vault = open_vault(master_password.as_str(), vault).unwrap();
    let mut new_entry = NewEntry::new(password);

    new_entry = new_entry.with_title(title);
    if let Some(username) = username {
        new_entry = new_entry.with_username(username);
    }
    if let Some(url) = url {
        new_entry = new_entry.with_url(url);
    }
    if let Some(notes) = notes {
        new_entry = new_entry.with_notes(notes);
    }
    if let Some(totp) = totp {
        new_entry = new_entry.with_totp(totp);
    }
    if let Some(group) = group {
        let group = vault.get_group_by_name(group.as_str());
        if group.is_err() {
            println!("{:?}", group.err());
            println!("group not found, returning");
            return;
        }
        new_entry = new_entry.with_parent_group(group.unwrap().id());
    }
    println!("{:?}", new_entry);
    let entry_id = vault.add_entry(new_entry).unwrap();
    vault.save(master_password.as_str());
}
pub fn handle_edit_entry(
    vault: PathBuf,
    master_password: String,
    id: String,
    password: String,
    title: Option<String>,
    username: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    totp: Option<String>,
) {
    let mut vault = open_vault(master_password.as_str(), vault).unwrap();

    let mut edit_entry = UpdateEntry::new();

    if let Some(title) = title {
        edit_entry = edit_entry.with_title(title);
    }
    if let Some(username) = username {
        edit_entry = edit_entry.with_title(username);
    }
    if let Some(url) = url {
        edit_entry = edit_entry.with_title(url);
    }
    if let Some(notes) = notes {
        edit_entry = edit_entry.with_title(notes);
    }
    if let Some(totp) = totp {
        edit_entry = edit_entry.with_title(totp);
    }

    println!("{:?}", edit_entry);

    vault
        .update_entry(Uuid::from_str(id.as_str()).unwrap(), edit_entry)
        .unwrap();
    vault.save(master_password.as_str());
}
pub fn handle_show_entry(vault: PathBuf, master_password: String, id: String) {
    let vault = open_vault(master_password.as_str(), vault).unwrap();
    let entry = vault.get_entry_by_title(id.as_str()).unwrap();
    println!("{:?}", entry);
}
pub fn handle_remove_entry(vault: PathBuf, master_password: String, id: String) {
    let mut vault = open_vault(master_password.as_str(), vault).unwrap();
    let entry = vault
        .get_entry(Uuid::from_str(id.as_str()).unwrap())
        .unwrap();
    vault.delete_entry(entry.id().uuid());
    vault.save(master_password.as_str());
}
