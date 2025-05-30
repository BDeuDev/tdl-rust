use actix_web::{Responder, HttpResponse, web};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

pub async fn echo (req: web::Json<EchoRequest>) -> impl Responder {
    let msg = format!("Echo: {}", req.message);
    HttpResponse::Ok().body(msg)

}