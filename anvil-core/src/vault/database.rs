use std::{fs::File, path::PathBuf};

use keepass::{Database, DatabaseKey};

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
    let mut file = File::create(path.clone())?;
    let db = Database::new();
    db.save(&mut file, DatabaseKey::new().with_password(master_password))
        .map_err(DatabaseProcessingError::Save)?;
    let vault = Vault {
        database: db,
        path: path,
        dirty: false,
    };

    Ok(vault)
}
