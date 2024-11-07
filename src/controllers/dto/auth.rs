use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::domain;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginDTO {
    #[validate(email(code ="400", message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 6, code = "400", message = "Password must be at least 6 characters"))]
    pub password: String
}

#[derive(Serialize)]
pub struct AuthToken {
    pub access_token: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct CreateUserDTO {
    #[validate(email(code ="400", message = "Invalid email"))]
    pub email: String,

    #[validate(length(min = 6, code = "400", message = "Password must be at least 6 characters"))]
    pub password: String,

    pub first_name: String,
    pub last_name: String,
    pub permission_system_setting: bool,
    pub permission_schedule: bool,
    pub permission_temporary_schedule: bool,
    pub permission_post_setting: bool,
}

impl Into<domain::models::auth::CreateUser> for CreateUserDTO {
    fn into(self) -> domain::models::auth::CreateUser {
        domain::models::auth::CreateUser { 
            email: self.email,
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name,
            permission_system_setting: self.permission_system_setting,
            permission_schedule: self.permission_schedule,
            permission_temporary_schedule: self.permission_temporary_schedule,
            permission_post_setting: self.permission_post_setting
        }
    }
}
