use std::{fs::File, path::PathBuf};

use keepass::{Database, DatabaseKey, config::DatabaseConfig};

use crate::vault::{DatabaseProcessingError, Vault};

impl Vault {
    pub fn save(&mut self, master_password: &str) -> Result<(), DatabaseProcessingError> {
        if !self.dirty {
            return Ok(());
        }

        let temp_path = self.path.with_extension("tmp");
        let mut file = File::create(&temp_path).map_err(DatabaseProcessingError::Io)?;

        self.database
            .save(&mut file, DatabaseKey::new().with_password(master_password))
            .map_err(DatabaseProcessingError::Save)?;

        std::fs::rename(&temp_path, &self.path).map_err(DatabaseProcessingError::Io)?;

        self.dirty = false;
        Ok(())
    }
}

pub fn open_vault(master_password: &str, path: PathBuf) -> Result<Vault, DatabaseProcessingError> {
    let mut file = File::open(path.clone()).map_err(|e| DatabaseProcessingError::Io(e))?;
    let key = DatabaseKey::new().with_password(master_password);
    let db = Database::open(&mut file, key).map_err(|e| DatabaseProcessingError::Open(e))?;
    let vault = Vault {
        database: db,
        path: path,
        dirty: false,
    };
    Ok(vault)
}

pub fn create_vault(
    master_password: &str,
    path: PathBuf,
) -> Result<Vault, DatabaseProcessingError> {
    let mut file = File::create(path.with_extension("kdbx").clone())?;
    let mut db_config = DatabaseConfig::default();

    db_config.kdf_config = keepass::config::KdfConfig::Argon2id {
        iterations: 80,
        memory: 65536 * 1024,
        parallelism: 4,
        version: argon2::Version::Version13,
    };
    let db = Database::with_config(db_config);
    db.save(&mut file, DatabaseKey::new().with_password(master_password))
        .map_err(DatabaseProcessingError::Save)?;
    let vault = Vault {
        database: db,
        path: path.with_extension("kdbx"),
        dirty: false,
    };

    Ok(vault)
}
