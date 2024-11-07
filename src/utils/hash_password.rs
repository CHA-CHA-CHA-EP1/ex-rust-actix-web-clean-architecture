use argon2::{self, Config};

use crate::config::config::CONFIG;

pub fn hash(password: &str) -> Result<String, String> {
    let salt = &CONFIG.salt;
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).map_err(|e| {
        "Failed to hash password".to_string() 
    })
}

pub fn verify(hash_password: &str, password: &str) -> Result<bool, String> {
    argon2::verify_encoded(hash_password, password.as_bytes()).map_err(|_| "".to_string())
}
