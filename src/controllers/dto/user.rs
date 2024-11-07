use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::domain;

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl From<domain::models::user::User> for UserDTO {
    fn from(value: domain::models::user::User) -> UserDTO {
        UserDTO {
            email: value.username,
            first_name: value.first_name,
            last_name: value.last_name,
        }
    }
}


#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserDTO {
    pub first_name: String,
    pub last_name: String,
    pub permission_system_setting: bool,
    pub permission_schedule: bool,
    pub permission_temporary_schedule: bool,
    pub permission_post_setting: bool,
}
