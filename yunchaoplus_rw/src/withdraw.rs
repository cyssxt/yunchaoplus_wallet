use actix_web::web;
use actix_web::{get, post, put, HttpResponse, Responder};

#[post("/wallets/{wallet_id}/withdraws")]
pub async fn create_withdraw(web::Path(wallet_id): web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/wallets/{wallet_id}/withdraws/{id}")]
pub async fn get_withdraw(
    web::Path((wallet_id, id)): web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::NoContent()
}

#[put("/wallets/{wallet_id}/withdraws/{id}")]
pub async fn update_withdraw(
    web::Path((wallet_id, id)): web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/wallets/{wallet_id}/withdraws")]
pub async fn get_withdraw_list(web::Path(wallet_id): web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}
