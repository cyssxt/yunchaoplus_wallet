use actix_web::{get, post, Responder};

#[post("/wallets/${wallet_id}/recharges")]
pub async fn create_recharge() -> impl Responder {
    unimplemented!()
}

#[get("/wallets/${wallet_id}/recharges/${id}")]
pub async fn get_recharge() -> impl Responder {
    unimplemented!()
}

#[get("/wallets/${wallet_id}/recharges")]
pub async fn get_recharge_list() -> impl Responder {
    unimplemented!()
}
