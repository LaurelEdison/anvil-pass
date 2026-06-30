use std::{path::PathBuf, str::FromStr};
mod cli;

use clap::Parser;
// TODO replace all unwrap with actual error handling

use crate::cli::{
    Cli,
    Commands::{Add, Backup, Edit, Info, Init, Ls, Mkdir, Remove, Rmdir, Search, Show, Tree},
    entries::{handle_add_entry, handle_edit_entry, handle_remove_entry, handle_show_entry},
    groups::{handle_add_group, handle_remove_group},
    handle_init,
    view::{handle_backup, handle_info, handle_list, handle_search, handle_tree},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        run_cli();
        return;
    }
    println!("tui not implemented yet, please provide argument")
}

fn run_cli() {
    let cli = Cli::parse();
    let result = match cli.command {
        Rmdir {
            vault,
            master_password,
            id,
        } => handle_remove_group(vault, master_password, id),
        Remove {
            vault,
            master_password,
            id,
        } => handle_remove_entry(vault, master_password, id),
        Search {
            vault,
            master_password,
            query,
        } => handle_search(vault, master_password, query),
        Show {
            vault,
            master_password,
            id,
        } => handle_show_entry(vault, master_password, id),
        Ls {
            vault,
            master_password,
            group,
        } => handle_list(vault, master_password, group),
        Backup {
            vault,
            master_password,
            path,
        } => handle_backup(vault, master_password, path),
        Tree {
            vault,
            master_password,
        } => handle_tree(vault, master_password),
        Add {
            vault,
            master_password,
            password,
            title,
            username,
            url,
            notes,
            totp,
            group,
        } => handle_add_entry(
            vault,
            master_password,
            password,
            title,
            username,
            url,
            notes,
            totp,
            group,
        ),
        Mkdir {
            vault,
            master_password,
            name,
            tags,
            notes,
            parent,
        } => handle_add_group(vault, master_password, name, tags, notes, parent),
        Init { password, name } => handle_init(password, name),
        Edit {
            vault,
            id,
            master_password,
            password,
            title,
            username,
            url,
            notes,
            totp,
        } => handle_edit_entry(
            vault,
            master_password,
            id,
            password,
            title,
            username,
            url,
            notes,
            totp,
        ),
        Info {
            vault,
            master_password,
        } => handle_info(vault, master_password),
    };
}
