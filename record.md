# 数据库的配置文件(.env)

## 准备 postgres 数据库
启动 posgres 数据库服务，并利用 sqlx database create 来创建数据库:
```
DATABASE_URL=postgres://localhost/test-sqlx-tokio sqlx database create
```
上述命令会创建名为 test-sqlx-tokio 的数据库。

## 创建 migration
sqlx 提供了 migration 的功能方便我们管理数据库的 schema 变更。

利用 sqlx migrate add 命令来创建 migration:
```
DATABASE_URL=postgres://localhost/test-sqlx-tokio sqlx migrate add customers
```
此时会在 migration 目录创建 <timestamp>-<name>.sql 格式的文件，我们可以把相应 SQL 语句放在这里。

## 运行 migration
利用 sqlx migrate run 来创建 migration，这条命令会执行我们刚刚添加的 sql 语句，从而完成 customer 表的创建。

```
DATABASE_URL=postgres://localhost/test-sqlx-tokio sqlx migrate run
```
登陆 test-sqlx-tokio 这个数据库，查看 customers 这个表。

##