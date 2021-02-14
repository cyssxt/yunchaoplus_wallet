# 项目名称

对象 & 提现对象 API

## 项目目录结构

- source root: src/
- db initialization: init.sql
- dotenv configuration: .env

## 数据定义

[init.sql](./init.sql)

tables:
- recharge
- withdraw

types:
- obj_type
- status

## API 定义

错误 code :
- `recharge_create_fail`
- `recharge_not_found`
- `recharge_list_not_found`
- `withdraw_creation_failed`
- `withdraw_not_found`
- `invalid_withdraw_update_status`
- `update_withdraw_failed`
- `withdraw_list_not_found`
- `invalid_paging_query`

## 开发环境搭建

1. use `init.sql` create tables
2. update onfiguration file `.env`

## CI/CD

### Branch

feat_withdraw

### ConfigMap

dotenv configuration: .env

example:
```dotenv
PG__HOST=localhost
PG__USER=curdata
PG__PASSWORD=curdata
PG__DBNAME=curdata
PG__POOL__MAX_SIZE=16
PG__POOL__TIMEOUTS__WAIT__SECS=5
PG__POOL__TIMEOUTS__WAIT__NANOS=0
```

### Ports

8080