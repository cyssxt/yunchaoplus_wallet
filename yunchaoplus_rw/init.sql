create type obj_type as enum (
    'recharge', -- 充值对象
    'withdraw' -- 支付对象
    );

create type status as enum (
    'created', -- 已申请
    'pending', -- 处理中
    'succeeded', -- 完成
    'failed', -- 失败
    'canceled' -- 取消
    );

create table if not exists recharge
(
    id              varchar primary key, -- 对象id
    type            obj_type,            -- 值为recharge，表示此对象为充值对象
    created         timestamp,           -- 账号创建时的 Unix 时间戳。
    amount          int,                 -- 用户实际到账余额，单位为分（包含赠送金额和扣除用户手续费，例如充 100 送 20，则该值是 120；充 100 收 5 元用户手续费，则该值是 95）。
    recharge_amount int,                 -- 充值金额
    fee             int,                 -- 充值手续费
    succeeded       boolean,             -- 是否已充值成功。
    time_succeeded  timestamp,           -- 充值成功时间，用 Unix 时间戳表示。
    wallet_id       varchar,             -- 充值目标 `wallet_id` 对象的 `id`。
    description     varchar(255),        -- 附加说明，最多 255 个 Unicode 字符。
    extra           jsonb,               -- 扩展用户字段
    settle          varchar              -- 结算对象id
    );



create table if not exists withdraw
(
    id             varchar primary key, -- 对象id
    type           obj_type,            -- 值为withdraw，表示此对象为支付对象
    created        timestamp,           -- 账号创建时的 Unix 时间戳。
    extra          jsonb,               -- 扩展用户字段
    description    varchar(60),         -- 附加说明，最多 60 个 Unicode 字符。
    status         status,              -- 提现状态
    wallet_id      varchar,             -- 提现关联 `wallet` 对象的 `id`。
    settle         varchar,             -- 提现使用的 `settle` 对象的 `id`。
    time_canceled  timestamp,           -- 提现取消时间，用 Unix 时间戳表示。
    time_succeeded timestamp,           -- 提现成功时间，用 Unix 时间戳表示。
    amount         int                  -- 提现金额
    )