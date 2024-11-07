use std::sync::Arc;

use async_trait::async_trait;
use crate::{controllers::dto::{self}, domain::{self, models::auth::{EmailAddress, Password}, repositories::user::UserRepository, services::auth}, utils};

#[derive(Clone)]
pub struct AuthServiceImpl {
    pub user_repository: Arc<dyn UserRepository>
}

impl AuthServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        AuthServiceImpl {
            user_repository,
        }
    }
}

#[async_trait]
impl auth::AuthService for AuthServiceImpl {
    async fn login(&self, email: EmailAddress, password: Password) -> Result<dto::auth::AuthToken, String> {
        let user = self.user_repository.get_user_by_email(email).await?;

        if !utils::hash_password::verify(&user.password, password.as_str())? {
            return Err("Invalid email or password".to_string())
        }

        let access_token = utils::jwt::encode_jwt(user.username)?;

        let auth_token = dto::auth::AuthToken {
            access_token,
        };

        Ok(auth_token)
    }

    async fn register(&self, mut create_user: domain::models::auth::CreateUser) -> Result<(), String> {
        let email_address: EmailAddress = create_user.email.clone().into();

        if let Ok(_) = self.user_repository.get_user_by_email(email_address).await {
            return Err("User already exists".to_string())
        }
        
        let hash_password = utils::hash_password::hash(&create_user.password)?;
        create_user.password = hash_password;

        self.user_repository.create(create_user.into()).await
    }
}
