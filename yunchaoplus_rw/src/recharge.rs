use crate::model::{ErrorResponse, Recharge, SuccessResponse};
use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

/// | 字段名         | 类型        | 描述                                                         | 属性          |
/// | -------------- | ----------- | ------------------------------------------------------------ | ------------- |
/// | recharge_amount | int    | 充值金额                                     | required |
/// | settle          | string | 结算对象id                                   | required |
/// | description     | string | 附加说明，最多 255 个 Unicode 字符。         | optional |
/// | extra           | object      | 额外参数，具体渠道不同有所区别，参见额外参数                 | optional      |
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CreateReq {
    recharge_amount: i32,
    settle: String,
    description: Option<String>,
    extra: Option<serde_json::Value>,
}

#[post("/wallets/{wallet_id}/recharges")]
pub async fn create_recharge(
    pool: web::Data<Pool>,
    web::Path(wallet_id): web::Path<String>,
    web::Json(CreateReq {
        recharge_amount,
        settle,
        description,
        extra,
    }): web::Json<CreateReq>,
) -> HttpResponse {
    match Recharge::create_recharge(
        &pool,
        wallet_id.clone(),
        recharge_amount,
        settle,
        description,
        extra,
    )
    .await
    {
        Ok(recharge) => HttpResponse::Ok().json(SuccessResponse::new(recharge)),
        Err(e) => {
            error!("/wallets/{}/recharges: {}", wallet_id, e);
            HttpResponse::InternalServerError().json(ErrorResponse::code("recharge_create_fail"))
        }
    }
}

#[get("/wallets/${wallet_id}/recharges/${id}")]
pub async fn get_recharge() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/wallets/${wallet_id}/recharges")]
pub async fn get_recharge_list() -> impl Responder {
    HttpResponse::NoContent()
}
