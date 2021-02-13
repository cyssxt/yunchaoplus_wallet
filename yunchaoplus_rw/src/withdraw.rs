use crate::model::{ErrorResponse, Status, SuccessResponse, Withdraw, PagingQuery};
use actix_web::web;
use actix_web::{get, post, put, HttpResponse, Responder};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

/// | 字段名         | 类型        | 描述                                                         | 属性          |
/// | -------------- | ----------- | ------------------------------------------------------------ | ------------- |
/// | settle          | string | 结算对象id                                   | required |
/// | amount | int | 提现金额 | required |
/// | description     | string | 附加说明，最多 255 个 Unicode 字符。         | optional |
/// | extra           | object      | 额外参数，具体渠道不同有所区别，参见额外参数                 | optional      |
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateReq {
    settle: String,
    amount: i32,
    description: Option<String>,
    extra: Option<serde_json::Value>,
}

/// | 字段名         | 类型        | 描述                                                         | 属性          |
/// | -------------- | ----------- | ------------------------------------------------------------ | ------------- |
/// | status | string | 取值范围：确认为 `pending`，取消为 `canceled`。 | required |
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateReq {
    status: Status,
}

#[post("/wallets/{wallet_id}/withdraws")]
pub async fn create_withdraw(
    pool: web::Data<Pool>,
    web::Path(wallet_id): web::Path<String>,
    web::Json(CreateReq {
        settle,
        amount,
        description,
        extra,
    }): web::Json<CreateReq>,
) -> impl Responder {
    let result =
        Withdraw::create_withdraw(&pool, wallet_id.clone(), settle, amount, description, extra)
            .await;
    match result {
        Ok(withdraw) => HttpResponse::Ok().json(SuccessResponse::new(withdraw)),
        Err(e) => {
            error!("/wallets/{}/withdraws: {}", wallet_id, e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::code("withdraw_creation_failed"))
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
    web::Json(UpdateReq { status }): web::Json<UpdateReq>,
) -> HttpResponse {
    match status {
        Status::Pending | Status::Canceled => (),
        _ => {
            return HttpResponse::BadRequest()
                .json(ErrorResponse::code("invalid_withdraw_update_status"))
        }
    }
    match Withdraw::set_wallet_status(&pool, wallet_id.clone(), id.clone(), status).await {
        Ok(withdraw) => HttpResponse::Ok().json(SuccessResponse::new(withdraw)),
        Err(e) => {
            error!("/wallets/{}/withdraws/{}: {}", wallet_id, id, e);
            HttpResponse::InternalServerError().json(ErrorResponse::code("update_withdraw_failed"))
        }
    }
}

#[get("/wallets/{wallet_id}/withdraws")]
pub async fn get_withdraw_list(
    pool: web::Data<Pool>,
    web::Path(wallet_id): web::Path<String>,
    web::Query(paging): web::Query<PagingQuery>,
) -> HttpResponse {
    if !paging.is_valid() {
        return HttpResponse::BadRequest()
            .json(ErrorResponse::code("invalid_paging_query"))
    }
    match Withdraw::list_withdraw(&pool, wallet_id.clone(), paging.clone()).await {
        Ok(withdraw) => HttpResponse::Ok().json(SuccessResponse::new(withdraw)),
        Err(e) => {
            error!("/wallets/{}/recharges({:?}): {}", wallet_id, paging, e);
            HttpResponse::NotFound().json(ErrorResponse::code("withdraw_list_not_found"))
        }
    }
}
