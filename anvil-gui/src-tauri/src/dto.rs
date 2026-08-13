use anvil_core::vault::models::{EntryData, GroupData};

pub struct EntryDto {
    pub id: String,
    pub parent: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub totp: String,
}
pub struct GroupDto {
    pub id: String,
    pub parent: Option<String>,
    pub name: String,
    pub tags: Vec<String>,
    pub notes: String,
}

pub fn entry_data_to_entry_dto(entry_data: EntryData) -> EntryDto {
    EntryDto {
        id: entry_data.id.to_string(),
        parent: entry_data.parent.to_string(),
        title: entry_data.title.unwrap_or(String::default()),
        username: entry_data.username.unwrap_or(String::default()),
        password: entry_data.password.unwrap_or(String::default()),
        url: entry_data.url.unwrap_or(String::default()),
        notes: entry_data.notes.unwrap_or(String::default()),
        totp: entry_data.totp.unwrap_or(String::default()),
    }
}
pub fn group_data_to_group_dto(group_data: GroupData) -> GroupDto {
    GroupDto {
        id: group_data.id.to_string(),
        name: group_data.name,
        tags: group_data.tags,
        notes: group_data.notes.unwrap_or(String::default()),
        parent: group_data.parent.map(|id| id.to_string()),
    }
}
