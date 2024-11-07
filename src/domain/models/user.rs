use crate::controllers::dto;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Debug)]
pub struct UpdateUser {
    pub first_name: String,
    pub last_name: String,
    pub permission_system_setting: bool,
    pub permission_schedule: bool,
    pub permission_temporary_schedule: bool,
    pub permission_post_setting: bool,
}

impl From<dto::user::UpdateUserDTO> for UpdateUser {
    fn from(value: dto::user::UpdateUserDTO) -> UpdateUser {
        UpdateUser {
            first_name: value.first_name,
            last_name: value.last_name,
            permission_system_setting: value.permission_system_setting,
            permission_schedule: value.permission_schedule,
            permission_temporary_schedule: value.permission_temporary_schedule,
            permission_post_setting: value.permission_post_setting
        }
    }
}
