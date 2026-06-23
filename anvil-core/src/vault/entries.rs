use keepass::db::{Entry, EntryId, EntryMut, EntryRef, GroupId, fields};
use uuid::Uuid;

use crate::vault::{
    DatabaseProcessingError::{self, FailedToFindEntry, FailedToFindGroup, FailedToMoveEntry},
    Vault,
};

#[derive(Debug, Clone)]
pub struct NewEntry {
    pub title: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub totp: Option<String>,
    pub group: Option<GroupId>,
}

#[derive(Default, Debug, Clone)]
pub struct UpdateEntry {
    pub title: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub totp: Option<String>,
}

impl NewEntry {
    pub fn new(password: impl Into<String>) -> Self {
        Self {
            title: None,
            username: None,
            password: password.into(),
            url: None,
            notes: None,
            totp: None,
            group: None,
        }
    }
    pub fn with_parent_group(mut self, group: GroupId) -> Self {
        self.group = Some(group);
        self
    }
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
    pub fn with_totp(mut self, totp: impl Into<String>) -> Self {
        self.totp = Some(totp.into());
        self
    }
}

impl Vault {
    pub fn list_entries<'a>(&'a self) -> Vec<EntryRef<'a>> {
        self.database.iter_all_entries().collect()
    }
    pub fn move_entry(
        &mut self,
        src_entry_id: Uuid,
        dst_group_id: Uuid,
    ) -> Result<(), DatabaseProcessingError> {
        let mut src_entry = self.get_entry_mut(src_entry_id)?;

        src_entry
            .move_to(GroupId::from_uuid(dst_group_id))
            .map_err(|e| FailedToMoveEntry {
                src_id: src_entry_id,
                dst_id: dst_group_id,
                err: e,
            })?;
        self.dirty = true;
        Ok(())
    }
    pub fn get_entry_by_title(&self, p_title: &str) -> Result<Entry, DatabaseProcessingError> {
        self.database
            .iter_all_entries()
            .find(|e| e.get(fields::TITLE) == Some(p_title))
            .ok_or(DatabaseProcessingError::FailedToFindEntryByTitle(
                String::from(p_title),
            ))
            .map(|e| e.clone())
    }

    pub fn add_entry(&mut self, new_entry: NewEntry) -> Result<Uuid, DatabaseProcessingError> {
        let mut group = match new_entry.group {
            Some(group_id) => self
                .database
                .group_mut(group_id)
                .ok_or(FailedToFindGroup(group_id.uuid()))?,
            None => self.database.root_mut(),
        };
        let mut entry = group.add_entry();
        //password is a required parameter
        entry.set_unprotected(fields::PASSWORD, new_entry.password);
        for (field, value) in [
            (fields::URL, new_entry.url),
            (fields::USERNAME, new_entry.username),
            (fields::NOTES, new_entry.notes),
            (fields::TITLE, new_entry.title),
            (fields::OTP, new_entry.totp),
        ] {
            if let Some(v) = value {
                entry.set_unprotected(field, v);
            }
        }
        self.dirty = true;
        let id = entry.id().uuid();
        Ok(id)
    }
    pub fn get_entry(&self, entry_id: Uuid) -> Result<Entry, DatabaseProcessingError> {
        self.database
            .entry(EntryId::from_uuid(entry_id))
            .ok_or(DatabaseProcessingError::FailedToFindEntry(entry_id))
            .map(|e| e.clone())
    }
    pub fn get_entry_mut<'a>(
        &'a mut self,
        entry_id: Uuid,
    ) -> Result<EntryMut<'a>, DatabaseProcessingError> {
        self.database
            .entry_mut(EntryId::from_uuid(entry_id))
            .ok_or(DatabaseProcessingError::FailedToFindEntry(entry_id))
    }

    pub fn update_entry(
        &mut self,
        entry_id: Uuid,
        update_entry: UpdateEntry,
    ) -> Result<(), DatabaseProcessingError> {
        self.database
            .entry_mut(EntryId::from_uuid(entry_id))
            .ok_or(FailedToFindEntry(entry_id))?
            .track_changes()
            .edit(|e| {
                for (field, value) in [
                    (fields::URL, update_entry.url),
                    (fields::USERNAME, update_entry.username),
                    (fields::NOTES, update_entry.notes),
                    (fields::PASSWORD, update_entry.password),
                    (fields::TITLE, update_entry.title),
                    (fields::OTP, update_entry.totp),
                ] {
                    if let Some(v) = value {
                        e.set_unprotected(field, v);
                    }
                }
            });

        self.dirty = true;
        Ok(())
    }
    pub fn delete_entry(&mut self, entry_id: Uuid) -> Result<(), DatabaseProcessingError> {
        self.database
            .entry_mut(EntryId::from_uuid(entry_id))
            .ok_or(FailedToFindEntry(entry_id))?
            .remove();
        self.dirty = true;
        Ok(())
    }

    pub fn get_entries_by_group(&self, group_id: GroupId) -> Vec<Entry> {
        self.database
            .group(group_id)
            .map(|g| g.entries().map(|e| e.clone()).collect())
            .unwrap_or_default()
    }
}
