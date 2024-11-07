use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, TokenData};

use crate::{config::config::CONFIG, domain::models::jwt::Claims};

pub fn encode_jwt(email: String) -> Result<String, String>{
    let secret_key = &CONFIG.secret;
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600;
    
    let claims = Claims {
        sub: email,
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).map_err(|_| "Failed to JWT encode".to_string())
}

pub fn is_valid_jwt(token: &str) -> Result<TokenData<Claims>, String> {
    if token.is_empty() {
        return Err("Token is empty".to_string());
    }

    if !token.starts_with("Bearer ") {
        return Err("Token is not a Bearer token".to_string());
    }

    let token = token.replace("Bearer ", "");

    let secret_key = &CONFIG.secret;
    jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(secret_key.as_ref()), &jsonwebtoken::Validation::default()).map_err(|_| "Failed to decode JWT".to_string())
}
