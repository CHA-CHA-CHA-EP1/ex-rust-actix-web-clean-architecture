use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use auth_service::domain::services::user::UserService;
use auth_service::services::user::UserServiceImpl;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use sqlx::Pool;

use auth_service::controllers;
use auth_service::domain::repositories::user::UserRepository;
use auth_service::domain::services::auth::AuthService;
use auth_service::services::auth::AuthServiceImpl;
use auth_service::infrastructure::repositories::user::UserRepositoryImpl;
use auth_service::config;
use auth_service::middleware;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // force lazy_static to load the configuration
    let database_url = &config::config::CONFIG.database_url;

    // Database setup.
    let pool: Pool<Postgres> = match PgPoolOptions::new() 
        .max_connections(5)
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            println!("Database connected");
            pool
        },
        Err(e) => {
            panic!("Failed to connect to database: {}", e);
        }
    };

    let user_repository: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(Arc::new(pool.clone())));

    let auth_service: Arc<dyn AuthService> = Arc::new(AuthServiceImpl {
        user_repository: user_repository.clone(),
    }); 

    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl {
        user_repository: user_repository.clone(),
    });

    println!("Server started at http://0.0.0.0:8080");
    HttpServer::new(move || {
        let cors = Cors::default();
        App::new()
            .app_data(web::Data::from(auth_service.clone()))
            .app_data(web::Data::from(user_service.clone()))
            .wrap(cors)
            .route("/health-check", web::get().to(controllers::health_check::health_check))
            .service(
                web::scope("auth")
                    .route("/login", web::post().to(controllers::auth_controller::login::login))
                    .route("/register", web::post().to(controllers::auth_controller::register::register))
            )
            .service(
                web::scope("user")
                    .wrap(middleware::jwt::Authentication)
                    .route("", web::get().to(controllers::user_controller::get_user::get_user))
                    .route("", web::put().to(controllers::user_controller::update_user::update_user))
            )
    })
    .bind((config::config::CONFIG.server_host.clone(), config::config::CONFIG.server_port))?
    .run()
    .await
}
