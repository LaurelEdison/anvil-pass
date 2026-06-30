pub mod entries;
pub mod groups;
pub mod view;

use std::path::PathBuf;

use anvil_core::vault::database::create_vault;
use clap::{Parser, Subcommand};
// TODO replace all unwrap with actual error handling
// Long names for now while i figure out good ux for ts
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub vault: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    // Create a vault
    Init {
        #[arg(long)]
        password: String,
        #[arg(long)]
        name: String,
    },

    Add {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
        #[arg(long)]
        password: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        totp: Option<String>,
        #[arg(long)]
        group: Option<String>,
    },

    // Show entry data, name, pass, url, etc
    Show {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
        #[arg(long)]
        id: String,
    },

    // Edit entry
    Edit {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        id: String,
        #[arg(long)]
        master_password: String,

        #[arg(long)]
        password: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        totp: Option<String>,
    },

    // Remove entry
    Remove {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
        #[arg(long)]
        id: String,
    },

    Mkdir {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
        #[arg(long)]
        name: String,

        //TODO maybe use delimited strings
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        #[arg(long)]
        notes: Option<String>,

        #[arg(long)]
        parent: Option<String>,
    },

    // List all groups and entries from root
    // preserving structure
    Tree {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
    },
    // List entries within a group, maybe add a option to list all
    // preserving structure
    Ls {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
        #[arg(long)]
        group: String,
    },

    // Deletes group, maybe add a recursive option
    Rmdir {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
        #[arg(long)]
        id: String,
    },

    // Search for entries
    // TODO implement search api
    Search {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
        #[arg(long)]
        query: String,
    },

    // Database metadata
    Info {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
    },

    Backup {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        master_password: String,
        #[arg(long)]
        path: PathBuf,
    },
}

pub fn handle_init(master_password: String, name: String) {
    let path = PathBuf::from(name);
    create_vault(master_password.as_str(), path);
}
