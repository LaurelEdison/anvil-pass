use keepass::db::{EntryRef, GroupRef};
use uuid::Uuid;

pub struct EntryData {
    pub id: Uuid,
    pub parent: Uuid,
    pub title: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub totp: Option<String>,
}
pub struct GroupData {
    pub id: Uuid,
    pub parent: Option<Uuid>,
    pub name: String,
    pub tags: Vec<String>,
    pub notes: Option<String>,
}
pub fn entry_ref_to_entry_data(entry_ref: EntryRef) -> EntryData {
    EntryData {
        id: entry_ref.id().uuid(),
        parent: entry_ref.parent().id().uuid(),
        title: entry_ref.get_title().map(str::to_owned),
        password: entry_ref.get_password().map(str::to_owned),
        username: entry_ref.get_username().map(str::to_owned),
        url: entry_ref.get_url().map(str::to_owned),
        totp: entry_ref.get_raw_otp_value().map(str::to_owned),
        notes: entry_ref.get("Notes").map(str::to_owned),
    }
}

pub fn group_ref_to_group_data(group_ref: GroupRef) -> GroupData {
    GroupData {
        id: group_ref.id().uuid(),
        name: group_ref.name.to_owned(),
        tags: group_ref.tags.to_owned(),
        notes: group_ref.notes.to_owned(),
        parent: group_ref.parent().map(|parent| parent.id().uuid()),
    }
}

pub fn entry_ref_collection_to_entry_data_collection(entry_refs: Vec<EntryRef>) -> Vec<EntryData> {
    let mut result = Vec::new();
    for entry_ref in entry_refs {
        result.push(entry_ref_to_entry_data(entry_ref));
    }
    result
}

pub fn group_ref_collection_to_group_data_collection(group_refs: Vec<GroupRef>) -> Vec<GroupData> {
    let mut result = Vec::new();
    for group_ref in group_refs {
        result.push(group_ref_to_group_data(group_ref));
    }
    result
}
