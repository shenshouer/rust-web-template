# rust-web-template

This is a template project for rust web application with use `axum` as web framework, `tokio` as async runtime, `sqlx` as DAO level.
`mockall` for all unit tests

## Start db first

```
docker rm -f postgres
docker run -d --name postgres \
       -p 5432:5432 \
       -e POSTGRES_PASSWORD=p@ssword! \
       -e PGDATA=/var/lib/postgresql/data/pgdata \
       -v /mnt/c/Users/sope/workspaces/db/postgres/data:/var/lib/postgresql/data \
       postgres:9.6.24

```

### run unit tests

```
DATABASE_URL=postgres://postgres:p%40ssword%21@localhost/cashbook cargo t
```

### run app

```
# create database schema, you must install sqlx-cli first 
cargo install sqlx-cli

DATABASE_URL=postgres://postgres:p%40ssword%21@localhost/cashbook sqlx migrate run

# run http server
cp .env-sample .env
cargo run --bin server

# run with log
RUST_LOG=debug cargo run --bin server
```