use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{query_as, Pool, Postgres};
use crate::{controllers::user_controller::update_user, domain::{self, models::auth::EmailAddress, repositories::user::UserRepository}, infrastructure::{self, models::user::User}};

#[derive(Clone)]
pub struct UserRepositoryImpl {
    db: Arc<Pool<Postgres>>
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<Pool<Postgres>>) -> UserRepositoryImpl {
        UserRepositoryImpl {
            db
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, new_user: infrastructure::models::user::CreateUser) -> Result<(), String> {
        let query = r#"
            INSERT INTO users (username, password, first_name, last_name, first_name_eng, last_name_eng, permission_system_setting, permission_schedule, permission_temporary_schedule, permission_post_setting)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#;

        sqlx::query(query)
            .bind(&new_user.email)
            .bind(&new_user.password)
            .bind(&new_user.first_name)
            .bind(&new_user.last_name)
            .bind("")
            .bind("")
            .bind(new_user.permission_system_setting)
            .bind(new_user.permission_schedule)
            .bind(new_user.permission_temporary_schedule)
            .bind(new_user.permission_post_setting)
            .execute(&*self.db) // Execute the query
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_user_by_email(&self, email: EmailAddress) -> Result<domain::models::user::User, String> {
        let user = query_as!(
            User,
            r#"SELECT username, password, first_name, last_name FROM users WHERE username = $1"#,
            email.to_string(),
        )
        .fetch_one(&*self.db)
        .await
        .map_err(|err| {
            match err {
                sqlx::Error::RowNotFound => "Invalid email or password".to_string(),
                _ => {
                    println!("[LOG] database fail");
                    "internal server error".to_string()
                },
            }
        })?;
        Ok(user.into())
    }

    async fn update(&self, email: EmailAddress, update_user: infrastructure::models::user::UpdateUser) -> Result<(), String> {
        let query = r#"
            UPDATE users
            SET first_name = $1, last_name = $2, permission_system_setting = $3, permission_schedule = $4, permission_temporary_schedule = $5, permission_post_setting = $6
            WHERE username = $7
        "#;

        sqlx::query(query)
            .bind(&update_user.first_name)
            .bind(&update_user.last_name)
            .bind(update_user.permission_system_setting)
            .bind(update_user.permission_schedule)
            .bind(update_user.permission_temporary_schedule)
            .bind(update_user.permission_post_setting)
            .bind(email.to_string())
            .execute(&*self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
