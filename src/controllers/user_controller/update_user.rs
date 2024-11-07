use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;

use crate::{controllers::dto, domain::{error::CommonError, services::user::UserService}, extractor};

pub async fn update_user(
    user_extractor: extractor::user::UserExtractor,
    user_service: web::Data<dyn UserService>,
    request: web::Json<dto::user::UpdateUserDTO>
) -> impl Responder {
    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(
            CommonError {
                code: 400,
                message: e.to_string()
            }
        )
    }


    let email = user_extractor.email;
    let body = request.into_inner();

    match user_service.update_user(email.into(), body.into()).await {
        Ok(_) => {
            return HttpResponse::Ok().json(json!({
                "code": 200,
                "message": "OK"
            }))
        },
        Err(e) => {
            return HttpResponse::BadRequest().body(e)
        }
    }
}
