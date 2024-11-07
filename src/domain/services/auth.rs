use async_trait::async_trait;
use crate::{controllers::dto, domain::{self, models::auth::{EmailAddress, Password}}};

#[async_trait]
pub trait AuthService: 'static + Sync + Send {
    async fn login(&self, email: EmailAddress, password: Password) -> Result<dto::auth::AuthToken, String>;
    async fn register(&self, mut create_user: domain::models::auth::CreateUser) -> Result<(), String>;
}
