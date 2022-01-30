use super::{DbPool, Table};
use anyhow::Result;
use futures::TryStreamExt;
use sqlx::{Executor, Pool, Postgres};

#[derive(sqlx::FromRow)]
struct User {
    pub id: u32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub mobile: String,
}

impl Table for User {
    fn table_name(&self) -> &str {
        "users"
    }
}

struct ListOption {
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    mobile: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl ListOption {
    fn get_sql_condition(self) -> String {
        todo!()
    }
}

impl User {
    pub async fn create(self, pool: &DbPool) -> Result<Self> {
        let sql = "INSERT INTO users (username, first_name, last_name, email, mobie) VALUES ($1, $2, $3, $4, $5) RETURNING *;";
        let user = sqlx::query_as::<_, User>(sql)
            .bind(self.username)
            .bind(self.first_name)
            .bind(self.last_name)
            .bind(self.email)
            .bind(self.mobile)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn get(&mut self, pool: &DbPool, id: u32) -> Result<&Self> {
        let sql = "SELECT * FROM users WHERE id = $1";
        let user = sqlx::query_as::<_, User>(sql)
            .bind(id)
            .fetch_one(pool)
            .await?;
        *self = user;
        Ok(self)
    }

    pub async fn list(pool: &DbPool) -> Result<Vec<User>> {
        Ok(vec![])
    }
}
