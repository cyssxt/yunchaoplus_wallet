use crate::model::{ErrorResponse, SuccessResponse, Withdraw};
use actix_web::web;
use actix_web::{get, post, put, HttpResponse, Responder, Result};
use anyhow::Error;
use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};

/// | 字段名         | 类型        | 描述                                                         | 属性          |
/// | -------------- | ----------- | ------------------------------------------------------------ | ------------- |
/// | settle          | string | 结算对象id                                   | required |
/// | amount | int | 提现金额 | required |
/// | description     | string | 附加说明，最多 255 个 Unicode 字符。         | optional |
/// | extra           | object      | 额外参数，具体渠道不同有所区别，参见额外参数                 | optional      |
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CreateReq {
    settle: String,
    amount: i32,
    description: Option<String>,
    extra: Option<serde_json::Value>,
}

#[post("/wallets/{wallet_id}/withdraws")]
pub async fn create_withdraw(
    pool: web::Data<Pool>,
    web::Path(wallet_id): web::Path<String>,
    web::Json(CreateReq { settle, amount, description, extra }): web::Json<CreateReq>,
) -> impl Responder {
    let result = Withdraw::create_withdraw(&pool,
        wallet_id.clone(),
        settle,
        amount,
        description,
        extra
    ).await;
    match result {
        Ok(withdraw) => HttpResponse::Ok().json(SuccessResponse::new(withdraw)),
        Err(e) => {
            error!("/wallets/{}/withdraws: {}", wallet_id, e);
            HttpResponse::NotFound().json(ErrorResponse::code("withdraw_creation_failed"))
        }
    }
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
