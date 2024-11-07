use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::init().expect("Failed to load config");
}

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub secret: String,
    pub salt: String,
    pub server_port: u16,
    pub server_host: String
}

impl Config {
    pub fn init() -> Result<Config, String> {
        let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not found")?;
        let secret = env::var("SECRET").map_err(|_| "SECRET not found")?;
        let salt = env::var("SALT").map_err(|_| "SALT not found")?;
        let server_port = env::var("SERVER_PORT").map_err(|_| "SERVER_PORT not found")?
            .parse::<u16>().map_err(|_| "SERVER_PORT must be a number")?;
        let server_host = env::var("SERVER_HOST").map_err(|_| "SERVER_HOST not found")?;

        Ok(Config {
            database_url,
            secret,
            salt,
            server_port,
            server_host
        })
    }
}
