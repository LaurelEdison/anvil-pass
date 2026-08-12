use std::{path::PathBuf, str::FromStr};

use anvil_core::vault::{
    self,
    entries::{NewEntry, UpdateEntry},
    groups::NewGroup,
};

fn main() {
    let mut vault = vault::database::create_vault(
        "password",
        PathBuf::from_str("/mnt/c/Users/Ferdinand Laurel Edi/Desktop/testing.kdbx").unwrap(),
    )
    .unwrap();

    let group = vault.add_group(NewGroup::new("trying")).unwrap();

    vault
        .add_entry(
            NewEntry::new("ehhh just tryign")
                .with_username("username@gmail.com")
                .with_url("gmail.com")
                .with_parent_group(group)
                .with_title("1"),
        )
        .unwrap();
    vault
        .add_entry(
            NewEntry::new("ehhh just tryign")
                .with_username("username@gmail.com")
                .with_url("gmail.com")
                .with_parent_group(group)
                .with_title("2"),
        )
        .unwrap();

    vault
        .add_entry(
            NewEntry::new("ehhh just tryign")
                .with_username("username@gmail.com")
                .with_url("gmail.com")
                .with_title("3"),
        )
        .unwrap();
    let entry = vault
        .add_entry(
            NewEntry::new("ehhh just tryign")
                .with_username("username@gmail.com")
                .with_url("gmail.com")
                .with_title("4"),
        )
        .unwrap();

    vault
        .update_entry(
            entry,
            UpdateEntry {
                title: Some("edited_entry".to_string()),
                username: None,
                password: None,
                url: None,
                notes: None,
                totp: None,
            },
        )
        .unwrap();

    let entry = vault
        .add_entry(
            NewEntry::new("ehhh just tryign")
                .with_username("username@gmail.com")
                .with_url("gmail.com")
                .with_title("should not exist"),
        )
        .unwrap();

    vault.delete_entry(entry).unwrap();

    let groups = vault.list_groups();
    let entries = vault.list_entries();

    print!("Entries:\n\n");
    entries.iter().for_each(|e| println!("{:?}", e));

    print!("\n\nGroups:\n\n");
    groups.iter().for_each(|e| println!("{:?}", e));

    vault.save("password").unwrap();
}
