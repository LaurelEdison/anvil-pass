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
    pub parent: String,
    pub name: String,
    pub tags: Vec<String>,
    pub notes: String,
}

pub fn entry_data_to_entry_dto() {}
pub fn group_data_to_group_dto() {}
