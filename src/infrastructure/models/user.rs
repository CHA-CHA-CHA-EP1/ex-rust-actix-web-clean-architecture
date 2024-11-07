use crate::domain;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String
}

impl Into<domain::models::user::User> for User {
    fn into(self) -> domain::models::user::User {
        domain::models::user::User {
            username: self.username,
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name
        }
    }
} 

#[derive(Debug)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub permission_system_setting: bool,
    pub permission_schedule: bool,
    pub permission_temporary_schedule: bool,
    pub permission_post_setting: bool,
}

impl From<domain::models::auth::CreateUser> for CreateUser {
    fn from(value: domain::models::auth::CreateUser) -> Self {
        CreateUser {
            email: value.email,
            password: value.password,
            first_name: value.first_name,
            last_name: value.last_name,
            permission_system_setting: value.permission_system_setting,
            permission_schedule: value.permission_schedule,
            permission_temporary_schedule: value.permission_temporary_schedule,
            permission_post_setting: value.permission_post_setting
        }
    }
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

impl From<domain::models::user::UpdateUser> for UpdateUser {
    fn from(value: domain::models::user::UpdateUser) -> Self {
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
