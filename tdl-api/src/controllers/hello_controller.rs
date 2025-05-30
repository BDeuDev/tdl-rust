use actix_web::{Responder, HttpResponse};

use crate::services::hello_service;

pub async fn hello() -> impl Responder {
    let msg = hello_service::get_hello_message();
    HttpResponse::Ok().body(msg)
}