use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::{domain::services::user::UserService, extractor};

pub async fn get_user(
    user_extractor: extractor::user::UserExtractor,
    user_service: web::Data<dyn UserService>
) -> impl Responder {
    let email = user_extractor.email;

    match user_service.get_user_info(email.into()).await {
        Ok(v) => {
            return HttpResponse::Ok().json(json!({
                "code": 200,
                "message": "OK",
                "data": v
            }))
        },
        Err(e) => {
            return HttpResponse::BadRequest().body(e)
        }
    }
}
