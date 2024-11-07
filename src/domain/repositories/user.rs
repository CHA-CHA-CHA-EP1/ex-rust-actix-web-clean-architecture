use async_trait::async_trait;

use crate::{domain::{self, models::auth::EmailAddress}, infrastructure};

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn create(&self, create_user: infrastructure::models::user::CreateUser) -> Result<(), String>;
    async fn get_user_by_email(&self, email: EmailAddress) -> Result<domain::models::user::User, String>;
    async fn update(&self, email: EmailAddress, update_user: infrastructure::models::user::UpdateUser) -> Result<(), String>;
}
