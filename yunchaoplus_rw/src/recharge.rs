use actix_web::{get, post, Responder, HttpResponse};

#[post("/wallets/${wallet_id}/recharges")]
pub async fn create_recharge() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/wallets/${wallet_id}/recharges/${id}")]
pub async fn get_recharge() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/wallets/${wallet_id}/recharges")]
pub async fn get_recharge_list() -> impl Responder {
    HttpResponse::NoContent()
}
