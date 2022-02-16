# 数据库相关操作

1. 安装命令行工具

```
cargo install sqlx-cli
```

2. 添加`uuid generated`支持
```
 DATABASE_URL=postgres://postgres:p%40ssword%21@localhost/cashbook sqlx migrate add uuid_generate_support
 DATABASE_URL=postgres://postgres:p%40ssword%21@localhost/cashbook sqlx migrate run
 ```

3. 添加新的数据表

```
DATABASE_URL=postgres://postgres:p%40ssword%21@localhost/cashbook sqlx migrate add users
DATABASE_URL=postgres://postgres:p%40ssword%21@localhost/cashbook sqlx migrate run
```

## redis缓存

```
docker run --name redis -p 6379:6379 -d redis
```