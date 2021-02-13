use chrono::NaiveDateTime;
use deadpool_postgres::Pool;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use tokio_postgres::types::{FromSql, ToSql};
use tokio_postgres::Row;

use crate::error::DBError;

/// Success response wrapper
/// ```json
/// {
///     "code": 0,
///     "message": "success",
///     "data": {}
/// }
/// ```
#[derive(Serialize)]
pub struct SuccessResponse<T: Serialize> {
    code: u32,
    message: String,
    data: T,
}

/// Error response wrapper
#[derive(Serialize)]
pub struct ErrorResponse {
    code: String,
    message: String,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_owned(),
            data,
        }
    }
}

impl ErrorResponse {
    pub fn code(code: &str) -> Self {
        Self {
            code: code.to_string(),
            message: "failed".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "obj_type")]
/// Object type enum
pub enum ObjType {
    #[postgres(name = "recharge")]
    #[serde(rename = "recharge")]
    Recharge,
    #[postgres(name = "withdraw")]
    #[serde(rename = "withdraw")]
    Withdraw,
}

#[derive(Copy, Clone, Debug, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "status")]
/// Status enum
pub enum Status {
    #[postgres(name = "created")]
    #[serde(rename = "created")]
    Created,
    #[postgres(name = "pending")]
    #[serde(rename = "pending")]
    Pending,
    #[postgres(name = "succeeded")]
    #[serde(rename = "succeeded")]
    Succeeded,
    #[postgres(name = "failed")]
    #[serde(rename = "failed")]
    Failed,
    #[postgres(name = "canceled")]
    #[serde(rename = "canceled")]
    Canceled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PagingQuery {
    page: u64,
    count: u64,
    #[serde(
        default,
        serialize_with = "timestamp_ser_option",
        deserialize_with = "timestamp_de_option"
    )]
    begin_time: Option<NaiveDateTime>,
    #[serde(
        default,
        serialize_with = "timestamp_ser_option",
        deserialize_with = "timestamp_de_option"
    )]
    end_time: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize)]
/// Recharge object
pub struct Recharge {
    /// 对象id
    id: String,
    /// 值为recharge，表示此对象为充值对象
    #[serde(rename = "type")]
    _type: ObjType,
    /// 账号创建时的 Unix 时间戳
    #[serde(serialize_with = "timestamp_ser", deserialize_with = "timestamp_de")]
    created: NaiveDateTime,
    /// 用户实际到账余额，单位为分（包含赠送金额和扣除用户手续费，例如充 100 送 20，则该值是 120；充 100 收 5 元用户手续费，则该值是 95）
    amount: i32,
    /// 充值金额
    recharge_amount: i32,
    /// 充值手续费
    fee: i32,
    /// 是否已充值成功
    succeeded: bool,
    /// 充值成功时间，用 Unix 时间戳表示
    #[serde(
        serialize_with = "timestamp_ser_option",
        deserialize_with = "timestamp_de_option"
    )]
    time_succeeded: Option<NaiveDateTime>,
    /// 充值目标 `wallet_id` 对象的 `id`
    wallet_id: String,
    /// 附加说明，最多 255 个 Unicode 字符
    description: Option<String>,
    /// 扩展用户字段
    extra: Option<serde_json::Value>,
    /// 结算对象id
    settle: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Withdraw object
pub struct Withdraw {
    /// 对象id
    id: String,
    /// 值为withdraw，表示此对象为支付对象
    #[serde(rename = "type")]
    _type: ObjType,
    /// 账号创建时的 Unix 时间戳
    #[serde(serialize_with = "timestamp_ser", deserialize_with = "timestamp_de")]
    created: NaiveDateTime,
    /// 扩展用户字段
    extra: Option<serde_json::Value>,
    /// 附加说明，最多 60 个 Unicode 字符
    description: Option<String>,
    /// 提现状态，已申请：created，处理中：pending，完成：succeeded，失败：failed，取消：canceled。
    status: Status,
    /// 提现关联 wallet 对象的 id
    wallet_id: String,
    /// 提现使用的 settle 对象的 id
    settle: String,
    /// 提现取消时间，用 Unix 时间戳表示
    #[serde(
        serialize_with = "timestamp_ser_option",
        deserialize_with = "timestamp_de_option"
    )]
    time_canceled: Option<NaiveDateTime>,
    /// 提现成功时间，用 Unix 时间戳表示
    #[serde(
        serialize_with = "timestamp_ser_option",
        deserialize_with = "timestamp_de_option"
    )]
    time_succeeded: Option<NaiveDateTime>,
    /// 提现金额
    amount: i32,
}

/// Manually mapping from postgres row to Recharge
impl TryFrom<Row> for Recharge {
    type Error = tokio_postgres::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            _type: row.try_get("type")?,
            created: row.try_get("created")?,
            amount: row.try_get("amount")?,
            recharge_amount: row.try_get("recharge_amount")?,
            fee: row.try_get("fee")?,
            succeeded: row.try_get("succeeded")?,
            time_succeeded: row.try_get("time_succeeded")?,
            wallet_id: row.try_get("wallet_id")?,
            description: row.try_get("description")?,
            extra: row.try_get("extra")?,
            settle: row.try_get("settle")?,
        })
    }
}

/// Manually mapping from postgres row to Withdraw
impl TryFrom<Row> for Withdraw {
    type Error = tokio_postgres::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            _type: row.try_get("type")?,
            created: row.try_get("created")?,
            extra: row.try_get("extra")?,
            description: row.try_get("description")?,
            status: row.try_get("status")?,
            wallet_id: row.try_get("wallet_id")?,
            settle: row.try_get("settle")?,
            time_canceled: row.try_get("time_canceled")?,
            time_succeeded: row.try_get("time_succeeded")?,
            amount: row.try_get("amount")?,
        })
    }
}

impl PagingQuery {
    pub fn is_valid(&self) -> bool {
        if self.page == 0 {
            return false
        }
        if self.count == 0 {
            return false
        }
        true
    }
}

impl Recharge {
    /// Create a new `Recharge` object
    pub async fn create_recharge(
        pool: &Pool,
        wallet_id: String,
        recharge_amount: i32,
        settle: String,
        description: Option<String>,
        extra: Option<serde_json::Value>,
    ) -> Result<Self, DBError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                r#"
            insert into recharge
                (id, wallet_id, recharge_amount, settle, amount, description, extra)
            values (uuid_generate_v4(), $1, $2, $3, $2, $4, $5)
            returning *
        "#,
            )
            .await?;
        let row = client
            .query_one(
                &stmt,
                &[&wallet_id, &recharge_amount, &settle, &description, &extra],
            )
            .await?;
        Ok(Self::try_from(row)?)
    }

    /// Get a `Recharge` object by wallet id and self id
    pub async fn get_by_wallet_id(
        pool: &Pool,
        wallet_id: String,
        id: String,
    ) -> Result<Self, DBError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                r#"
            select * from recharge
            where wallet_id = $1 and id = $2
            limit 1
        "#,
            )
            .await?;
        let row = client.query_one(&stmt, &[&wallet_id, &id]).await?;
        Ok(Self::try_from(row)?)
    }

    /// List `Recharge` objects with paging
    pub async fn list_recharge(
        pool: &Pool,
        wallet_id: String,
        paging: PagingQuery,
    ) -> Result<Vec<Recharge>, DBError> {
        let client = pool.get().await?;
        let offset = ((paging.page - 1) * paging.count) as i64;
        let limit = paging.count as i64;
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![&wallet_id, &limit, &offset];
        let fragment: &str = if paging.begin_time.is_some() && paging.end_time.is_some() {
            params.push(paging.begin_time.as_ref().unwrap());
            params.push(paging.end_time.as_ref().unwrap());
            "and created >= $4 and created <= $4"
        } else if paging.begin_time.is_some() {
            params.push(paging.begin_time.as_ref().unwrap());
            "and created >= $4"
        } else if paging.end_time.is_some() {
            params.push(paging.end_time.as_ref().unwrap());
            "and created <= $4"
        } else {
            ""
        };
        let stmt = client
            .prepare(
                format!(
                    "select * from recharge where wallet_id = $1 {} limit $2 offset $3",
                    fragment
                )
                .as_str(),
            )
            .await?;
        let rows = client.query(&stmt, &params).await?;
        let recharges: Result<Vec<Self>, tokio_postgres::Error> = rows.into_iter().map(Self::try_from).collect();
        Ok(recharges?)
    }
}

impl Withdraw {
    /// Create a new `Withdraw` object
    pub async fn create_withdraw(
        pool: &Pool,
        wallet_id: String,
        settle: String,
        amount: i32,
        description: Option<String>,
        extra: Option<serde_json::Value>,
    ) -> Result<Self, DBError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                r#"
                insert into withdraw
                    (id, wallet_id, settle, amount, description, extra)
                values (uuid_generate_v4(), $1, $2, $3, $4, $5)
                returning *;
                "#,
            )
            .await?;
        let row = client
            .query_one(&stmt, &[&wallet_id, &settle, &amount, &description, &extra])
            .await?;
        Ok(Self::try_from(row)?)
    }

    /// Get a `Withdraw` object by wallet id and self id
    pub async fn get_by_wallet_id(
        pool: &Pool,
        wallet_id: String,
        id: String,
    ) -> Result<Self, DBError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                r#"
            select * from withdraw
            where wallet_id = $1 and id = $2
            limit 1
        "#,
            )
            .await?;
        let row = client.query_one(&stmt, &[&wallet_id, &id]).await?;
        Ok(Self::try_from(row)?)
    }

    /// Update a `Withdraw` object status
    pub async fn set_wallet_status(
        pool: &Pool,
        wallet_id: String,
        id: String,
        status: Status,
    ) -> Result<Self, DBError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                r#"
            update withdraw
            set status = $3::status
            where wallet_id = $1 and id = $2
            returning *;
            "#,
            )
            .await?;
        let row = client.query_one(&stmt, &[&wallet_id, &id, &status]).await?;
        Ok(Self::try_from(row)?)
    }

    /// List `Withdraw` objects with paging
    pub async fn list_withdraw(
        pool: &Pool,
        wallet_id: String,
        paging: PagingQuery,
    ) -> Result<Vec<Self>, DBError> {
        let client = pool.get().await?;
        let offset = ((paging.page - 1) * paging.count) as i64;
        let limit = paging.count as i64;
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![&wallet_id, &limit, &offset];
        let fragment: &str = if paging.begin_time.is_some() && paging.end_time.is_some() {
            params.push(paging.begin_time.as_ref().unwrap());
            params.push(paging.end_time.as_ref().unwrap());
            "and created >= $4 and created <= $4"
        } else if paging.begin_time.is_some() {
            params.push(paging.begin_time.as_ref().unwrap());
            "and created >= $4"
        } else if paging.end_time.is_some() {
            params.push(paging.end_time.as_ref().unwrap());
            "and created <= $4"
        } else {
            ""
        };
        let stmt = client
            .prepare(
                format!(
                    "select * from withdraw where wallet_id = $1 {} limit $2 offset $3",
                    fragment
                )
                    .as_str(),
            )
            .await?;
        let rows = client.query(&stmt, &params).await?;
        let withdraws: Result<Vec<Self>, tokio_postgres::Error> = rows.into_iter().map(Self::try_from).collect();
        Ok(withdraws?)
    }
}

#[doc(hidden)]
/// deserializer of `NaiveDateTime` from `i64`
fn timestamp_de<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    i64::deserialize(deserializer).map(|ts| NaiveDateTime::from_timestamp(ts, 0))
}

#[doc(hidden)]
/// deserializer of `Option<NaiveDateTime>` from `Option<i64>`
fn timestamp_de_option<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<i64>::deserialize(deserializer)
        .map(|ts_option| ts_option.map(|ts| NaiveDateTime::from_timestamp(ts, 0)))
}

#[doc(hidden)]
/// serializer of `NaiveDateTime` to `i64`
fn timestamp_ser<S>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(time.timestamp())
}

#[doc(hidden)]
/// serializer of `Option<NaiveDateTime>` to `i64` or `None`
fn timestamp_ser_option<S>(time: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match time {
        None => serializer.serialize_none(),
        Some(time) => serializer.serialize_i64(time.timestamp()),
    }
}

#[doc(hidden)]
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use tokio::runtime::Runtime;
    use tokio_postgres::{Error, NoTls};

    lazy_static! {
        static ref RUNTIME: Runtime = Runtime::new().unwrap();
    }
}
