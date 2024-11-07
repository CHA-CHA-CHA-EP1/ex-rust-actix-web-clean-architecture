use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;

use crate::{controllers::dto::auth::CreateUserDTO, domain::{error::CommonError, services::auth::AuthService}};

pub async fn register(
    auth_service: web::Data<dyn AuthService> ,
    request: web::Json<CreateUserDTO>
) -> impl Responder {
    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(
            CommonError {
                code: 400,
                message: e.to_string()
            }
        )
    }

    let body = request.into_inner();

    match auth_service.register(body.into()).await {
        Ok(_) => {
           return HttpResponse::Created().json(json!({
               "code": 200,
               "message": "OK",
           }))
        },
        Err(e) => {
            return HttpResponse::BadRequest().json(
                CommonError {
                    code: 400,
                    message: e.to_string()
                }
            )
        }
    }
}
