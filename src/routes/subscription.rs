use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<SubscribeFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
