use actix_web::{FromRequest, Error as ActixError};
use futures_util::future::Ready;
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserExtractor {
    pub email: String,
}


impl FromRequest for UserExtractor {
    type Error = ActixError; 
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        let headers = req.headers().clone();
        let auth_header = headers.get("authorization").map(|v| v.to_str().unwrap().to_string());
        let token = utils::jwt::is_valid_jwt(auth_header.unwrap_or("".to_string()).as_str());

        let token = token.unwrap();
        let email = token.claims.sub;

        futures_util::future::ready(Ok(UserExtractor {
            email,
        }))
    }
}
