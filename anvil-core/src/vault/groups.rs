use keepass::db::{GroupId, GroupMut, GroupRef};
use uuid::Uuid;

use crate::vault::{
    DatabaseProcessingError::{self, FailedToFindGroup, FailedToMoveGroup},
    Vault,
};

#[derive(Debug, Clone)]
pub struct NewGroup {
    pub name: String,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub parent: Option<GroupId>,
}

#[derive(Debug, Clone)]
pub struct UpdateGroup {
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
}

impl NewGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tags: None,
            notes: None,
            parent: None,
        }
    }
    pub fn with_tags(mut self, tags: impl Into<Vec<String>>) -> Self {
        self.tags = Some(tags.into());
        self
    }
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
    pub fn with_parent_group(mut self, parent: impl Into<GroupId>) -> Self {
        self.parent = Some(parent.into());
        self
    }
}

impl Vault {
    pub fn list_groups<'a>(&'a self) -> Vec<GroupRef<'a>> {
        self.database.iter_all_groups().collect()
    }
    pub fn delete_group(&mut self, group_id: Uuid) -> Result<(), DatabaseProcessingError> {
        self.database
            .group_mut(GroupId::from_uuid(group_id))
            .ok_or(FailedToFindGroup(group_id))?
            .remove();
        self.dirty = true;
        Ok(())
    }

    pub fn update_group(
        &mut self,
        group_id: Uuid,
        update_group: UpdateGroup,
    ) -> Result<(), DatabaseProcessingError> {
        self.database
            .group_mut(GroupId::from_uuid(group_id))
            .ok_or(FailedToFindGroup(group_id))?
            .edit_tracking(|e| {
                if let Some(tags) = update_group.tags {
                    e.tags = tags;
                }
                if let Some(name) = update_group.name {
                    e.name = name;
                }
                e.notes = update_group.notes;
            });
        self.dirty = true;
        Ok(())
    }
    pub fn move_group(
        &mut self,
        src_group_id: Uuid,
        dst_group_id: Uuid,
    ) -> Result<(), DatabaseProcessingError> {
        let mut src_group = self.get_group_mut(src_group_id)?;
        src_group
            .move_to(GroupId::from_uuid(dst_group_id))
            .map_err(|e| FailedToMoveGroup {
                src_id: src_group_id,
                dst_id: dst_group_id,
                err: e,
            })?;
        self.dirty = true;
        Ok(())
    }
    pub fn add_group(&mut self, new_group: NewGroup) -> Result<Uuid, DatabaseProcessingError> {
        let mut parent_group = match new_group.parent {
            Some(group_id) => self
                .database
                .group_mut(group_id)
                .ok_or(FailedToFindGroup(group_id.uuid()))?,
            None => self.database.root_mut(),
        };

        let mut group = parent_group.add_group();

        group.name = new_group.name;

        if let Some(tags) = new_group.tags {
            group.tags = tags;
        }
        group.notes = new_group.notes;

        self.dirty = true;
        Ok(group.id().uuid())
    }

    pub fn get_group<'a>(
        &'a self,
        group_id: Uuid,
    ) -> Result<GroupRef<'a>, DatabaseProcessingError> {
        self.database
            .group(GroupId::from_uuid(group_id))
            .ok_or(DatabaseProcessingError::FailedToFindGroup(group_id))
    }

    pub fn get_group_mut<'a>(
        &'a mut self,
        group_id: Uuid,
    ) -> Result<GroupMut<'a>, DatabaseProcessingError> {
        self.database
            .group_mut(GroupId::from_uuid(group_id))
            .ok_or(DatabaseProcessingError::FailedToFindGroup(group_id))
    }
}
