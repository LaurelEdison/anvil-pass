use std::path::PathBuf;

use anvil_core::vault::database::open_vault;
// TODO replace all unwrap with actual error handling

pub fn handle_list(vault: PathBuf, master_password: String, group: String) {
    let vault = open_vault(master_password.as_str(), vault).unwrap();
    let group = vault.get_group_by_name(&group).unwrap();
    let tree = vault.as_tree(Some(group.id().uuid())).unwrap();

    for node in tree.nodes {
        if node.is_group() {
            println!(
                "{}{}/",
                " ".repeat(node.get_depth()),
                node.get_display_name()
            );
        } else {
            println!(
                "{}{}",
                " ".repeat(node.get_depth()),
                node.get_display_name()
            );
        }
    }
}
pub fn handle_search(vault: PathBuf, master_password: String, query: String) {
    let vault = open_vault(master_password.as_str(), vault).unwrap();
}

pub fn handle_backup(vault: PathBuf, master_password: String, path: PathBuf) {
    let vault = open_vault(master_password.as_str(), vault).unwrap();
    let tree = vault.as_tree(None).unwrap();
}
pub fn handle_tree(vault: PathBuf, master_password: String) {
    let vault = open_vault(master_password.as_str(), vault).unwrap();
    let tree = vault.as_tree(None).unwrap();
    for node in tree.nodes {
        if node.is_group() {
            println!(
                "{}{}/",
                " ".repeat(node.get_depth()),
                node.get_display_name()
            );
        } else {
            println!(
                "{}{}",
                " ".repeat(node.get_depth()),
                node.get_display_name()
            );
        }
    }
}
pub fn handle_info(vault: PathBuf, master_password: String) {
    let vault = open_vault(master_password.as_str(), vault).unwrap();
    println!("{:?}", vault.database.meta)
}
