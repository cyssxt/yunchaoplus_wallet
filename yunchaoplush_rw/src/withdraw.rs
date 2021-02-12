use actix_web::{get, post, put, Responder};

#[post("/wallets/${wallet_id}/withdraws")]
pub async fn create_withdraw() -> impl Responder {
    unimplemented!()
}

#[get("/wallets/${wallet_id}/withdraws/${id}")]
pub async fn get_withdraw() -> impl Responder {
    unimplemented!()
}

#[put("/wallets/${wallet_id}/withdraws/${id}")]
pub async fn update_withdraw() -> impl Responder {
    unimplemented!()
}

#[get("/wallets/${wallet_id}/withdraws")]
pub async fn get_withdraw_list() -> impl Responder {
    unimplemented!()
}
