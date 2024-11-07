use std::sync::Arc;

use async_trait::async_trait;

use crate::{controllers::dto, domain::{self, repositories::user::UserRepository}};

#[derive(Clone)]
pub struct UserServiceImpl {
    pub user_repository: Arc<dyn UserRepository>
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl {
            user_repository,
        }
    }
}

#[async_trait]
impl domain::services::user::UserService for UserServiceImpl {
    async fn get_user_info(&self, email: domain::models::auth::EmailAddress) -> Result<dto::user::UserDTO, String> {
        let user = self.user_repository.get_user_by_email(email).await?;
        Ok(user.into())
    }

    async fn update_user(&self, email: domain::models::auth::EmailAddress, update_user: domain::models::user::UpdateUser) -> Result<(), String> {
        self.user_repository.update(email, update_user.into()).await?;
        Ok(())
    }
}
