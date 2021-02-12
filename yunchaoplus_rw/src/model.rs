use chrono::NaiveDateTime;
use tokio_postgres::types::{FromSql, ToSql};

#[derive(Copy, Clone, Debug, ToSql, FromSql)]
#[postgres(name = "obj_type")]
pub enum ObjType {
    #[postgres(name = "recharge")]
    Recharge,
    #[postgres(name = "withdraw")]
    Withdraw,
}

#[derive(Copy, Clone, Debug, ToSql, FromSql)]
#[postgres(name = "status")]
pub enum Status {
    #[postgres(name = "created")]
    Created,
    #[postgres(name = "pending")]
    Pending,
    #[postgres(name = "succeeded")]
    Succeeded,
    #[postgres(name = "failed")]
    Failed,
    #[postgres(name = "canceled")]
    Canceled,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct Recharge {
    /// 对象id
    id: String,
    /// 值为recharge，表示此对象为充值对象
    #[postgres(name = "type")]
    _type: String,
    /// 账号创建时的 Unix 时间戳
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
    time_succeeded: NaiveDateTime,
    /// 充值目标 `wallet_id` 对象的 `id`
    wallet_id: String,
    /// 附加说明，最多 255 个 Unicode 字符
    description: String,
    /// 扩展用户字段
    extra: serde_json::Value,
    /// 结算对象id
    settle: String,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct Withdraw {
    /// 对象id
    id: String,
    /// 值为withdraw，表示此对象为支付对象
    #[postgres(name = "type")]
    _type: String,
    /// 账号创建时的 Unix 时间戳
    created: NaiveDateTime,
    /// 扩展用户字段
    extra: serde_json::Value,
    /// 附加说明，最多 60 个 Unicode 字符
    description: String,
    /// 提现状态，已申请：created，处理中：pending，完成：succeeded，失败：failed，取消：canceled。
    status: Status,
    /// 提现关联 wallet 对象的 id
    wallet_id: String,
    /// 提现使用的 settle 对象的 id
    settle: String,
    /// 提现取消时间，用 Unix 时间戳表示
    time_canceled: NaiveDateTime,
    /// 提现成功时间，用 Unix 时间戳表示
    time_succeeded: NaiveDateTime,
    /// 提现金额
    amount: i32,
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;
    use tokio_postgres::{Error, NoTls};

    lazy_static! {
        static ref RUNTIME: Runtime = Runtime::new().unwrap();
    }

    #[test]
    fn test() {
        RUNTIME.block_on(test_write())
    }

    async fn test_write() {
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=curdata password=curdata", NoTls).await.unwrap();
    }
}
