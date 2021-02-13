use crate::model::{ErrorResponse, SuccessResponse, Withdraw};
use actix_web::web;
use actix_web::{get, post, put, HttpResponse, Responder, Result};
use anyhow::Error;
use deadpool_postgres::Pool;

#[post("/wallets/{wallet_id}/withdraws")]
pub async fn create_withdraw(
    pool: web::Data<Pool>,
    web::Path(wallet_id): web::Path<String>,
) -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/wallets/{wallet_id}/withdraws/{id}")]
pub async fn get_withdraw(
    pool: web::Data<Pool>,
    web::Path((wallet_id, id)): web::Path<(String, String)>,
) -> HttpResponse {
    match Withdraw::get_by_wallet_id(&pool, wallet_id.clone(), id.clone()).await {
        Ok(withdraw) => HttpResponse::Ok().json(SuccessResponse::new(withdraw)),
        Err(e) => {
            error!("/wallets/{}/withdraws/{}: {}", wallet_id, id, e);
            HttpResponse::NotFound().json(ErrorResponse::code("withdraw_not_found"))
        }
    }
}

#[put("/wallets/{wallet_id}/withdraws/{id}")]
pub async fn update_withdraw(
    pool: web::Data<Pool>,
    web::Path((wallet_id, id)): web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/wallets/{wallet_id}/withdraws")]
pub async fn get_withdraw_list(
    pool: web::Data<Pool>,
    web::Path(wallet_id): web::Path<String>,
) -> impl Responder {
    HttpResponse::NoContent()
}
