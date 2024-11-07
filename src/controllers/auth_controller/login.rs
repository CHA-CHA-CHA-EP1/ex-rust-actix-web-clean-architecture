use actix_web::{web::{self}, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;
use crate::{controllers::dto::auth::LoginDTO, domain::{error::CommonError, services::auth::AuthService}};

pub async fn login(
    auth_service: web::Data<dyn AuthService>,
    request: web::Json<LoginDTO>,
) -> impl Responder {
    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(
            CommonError {
                code: 400,
                message: e.to_string()
            }
        )
    }

   let body: LoginDTO = request.into_inner();

   match auth_service.login(body.email.into(), body.password.into()).await {
       Ok(v) => {
           return HttpResponse::Ok().json(json!({
               "code": 200,
               "message": "OK",
               "data": v
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
