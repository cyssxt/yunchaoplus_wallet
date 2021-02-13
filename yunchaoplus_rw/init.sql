drop table if exists recharge;
drop table if exists withdraw;
drop type if exists obj_type;
drop type if exists status;

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
    id              varchar primary key,                                -- 对象id
    type            obj_type
    default 'recharge'::obj_type not null
    check ( type = 'recharge'::obj_type ),                          -- 值为recharge，表示此对象为充值对象
    created         timestamp    default current_timestamp    not null, -- 账号创建时的 Unix 时间戳。
    amount          int                                       not null, -- 用户实际到账余额，单位为分（包含赠送金额和扣除用户手续费，例如充 100 送 20，则该值是 120；充 100 收 5 元用户手续费，则该值是 95）。
    recharge_amount int                                       not null, -- 充值金额
    fee             int          default 0                    not null, -- 充值手续费
    succeeded       boolean      default false                not null, -- 是否已充值成功。
    time_succeeded  timestamp    default null,                          -- 充值成功时间，用 Unix 时间戳表示。
    wallet_id       varchar                                   not null, -- 充值目标 `wallet_id` 对象的 `id`。
    description     varchar(255) default null,                          -- 附加说明，最多 255 个 Unicode 字符。
    extra           jsonb        default null,                          -- 扩展用户字段
    settle          varchar                                   not null  -- 结算对象id
    );

create unique index on recharge (wallet_id, id);



create table if not exists withdraw
(
    id             varchar primary key,                               -- 对象id
    type           obj_type
    default 'withdraw'::obj_type not null
    check ( type = 'withdraw'::obj_type ),                        -- 值为withdraw，表示此对象为支付对象
    created        timestamp   default current_timestamp    not null, -- 账号创建时的 Unix 时间戳。
    extra          jsonb       default null,                          -- 扩展用户字段
    description    varchar(60) default null,                          -- 附加说明，最多 60 个 Unicode 字符。
    status         status      default 'created'::status    not null, -- 提现状态
    wallet_id      varchar                                  not null, -- 提现关联 `wallet` 对象的 `id`。
    settle         varchar                                  not null, -- 提现使用的 `settle` 对象的 `id`。
    time_canceled  timestamp   default null,                          -- 提现取消时间，用 Unix 时间戳表示。
    time_succeeded timestamp   default null,                          -- 提现成功时间，用 Unix 时间戳表示。
    amount         int                                      not null  -- 提现金额
    );

create unique index on withdraw (wallet_id, id);