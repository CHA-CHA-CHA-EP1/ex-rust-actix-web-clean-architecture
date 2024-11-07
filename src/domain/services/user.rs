use async_trait::async_trait;

use crate::{controllers::{dto, user_controller::update_user}, domain};

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn get_user_info(&self, email: domain::models::auth::EmailAddress) -> Result<dto::user::UserDTO, String>;
    async fn update_user(&self, email: domain::models::auth::EmailAddress, update_user: domain::models::user::UpdateUser) -> Result<(), String>;
}
