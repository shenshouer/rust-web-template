use crate::{
    errors::Result,
    models::{
        auth::Credential,
        user::{CreateUser, User, UserOption},
    },
};
use axum::async_trait;
use chrono::Utc;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

/// user数据访问接口
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepo {
    async fn create(&self, user: CreateUser) -> Result<User>;
    async fn get(&self, id: Uuid) -> Result<User>;
    async fn get_by_email(&self, email: &str) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<User>;
    async fn update(&self, user: &User) -> Result<User>;
    async fn list(&self, fields: UserOption) -> Result<Vec<User>>;
    async fn authenticate(&self, credential: Credential) -> Result<User>;
}

#[derive(Clone)]
pub struct UserRepoImpl {
    pool: Arc<PgPool>,
}

impl UserRepoImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserRepoImpl { pool }
    }
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn create(&self, user: CreateUser) -> Result<User> {
        let sql = format!(
            "
            INSERT INTO {} (name, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            ",
            User::TABLE,
        );
        Ok(sqlx::query_as(&sql)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(Utc::now())
            .bind(Utc::now())
            .fetch_one(&*self.pool)
            .await?)
    }

    async fn get_by_email(&self, email: &str) -> Result<User> {
        let sql = format!("SELECT * FROM {} WHERE email = $1 LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql)
            .bind(email)
            .fetch_one(&*self.pool)
            .await?)
    }

    async fn get(&self, id: Uuid) -> Result<User> {
        let sql = format!("SELECT * FROM {} WHERE id = $1", User::TABLE);
        let user = sqlx::query_as::<_, User>(&sql)
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<User> {
        let sql = format!(r#"DELETE FROM {} WHERE id = $1 RETURNING *"#, User::TABLE);
        let user = sqlx::query_as(&sql).bind(id).fetch_one(&*self.pool).await?;
        Ok(user)
    }

    async fn update(&self, user: &User) -> Result<User> {
        let sql = format!(
            r#"
        UPDATE {} SET 
            name = $1,  
            email = $2,
            password = $3,
            updated_at = $4
            WHERE id = $5
            RETURNING *
            "#,
            User::TABLE
        );
        let user = sqlx::query_as::<_, User>(&sql)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password)
            .bind(Utc::now())
            .bind(&user.id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(user)
    }

    async fn list(&self, opts: UserOption) -> Result<Vec<User>> {
        let sql = format!("SELECT * FROM users {opts}");
        let rows = sqlx::query_as(&sql).fetch_all(&*self.pool).await?;
        Ok(rows)
    }

    async fn authenticate(&self, credential: Credential) -> Result<User> {
        let sql = format!(
            "SELECT true FROM {} WHERE email = $1 AND password = crypt($2, password) RETURNING *",
            User::TABLE
        );
        let user = sqlx::query_as(&sql)
            .bind(&credential.email)
            .bind(&credential.password)
            .fetch_one(&*self.pool)
            .await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    // #[traced_test]
    #[tokio::test]
    async fn test_user_repo() -> Result<()> {
        use super::*;
        use sqlx::postgres::PgPoolOptions;
        use std::sync::Arc;
        use tracing::info;
        // std::env::set_var("RUST_LOG", "debug");
        // tracing_subscriber::fmt::init();

        info!("starting create init pool ");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:p%40ssword%21@localhost")
            .await
            .unwrap();

        info!("starting check test database if exists and drop it ");
        sqlx::query("drop database if exists test_user_repo")
            .execute(&pool)
            .await
            .unwrap();

        info!("starting create new test database ");
        sqlx::query("create database test_user_repo")
            .execute(&pool)
            .await
            .unwrap();

        info!("starting create new db pool ");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:p%40ssword%21@localhost/test_user_repo")
            .await
            .unwrap();

        info!("starting migrate schemas ");
        sqlx::migrate!().run(&pool).await.unwrap();

        let sut = UserRepoImpl {
            pool: Arc::new(pool),
        };

        let create_entity = CreateUser {
            name: "fn1".to_string(),
            email: "email1".to_string(),
            password: "".to_string(),
        };

        info!("testing create new user ");
        let user = sut.create(create_entity.clone()).await.unwrap();

        assert_eq!(&user.name, &create_entity.name);
        assert!(!user.id.is_nil());

        info!("testing get user ");
        let mut get_user = sut.get(user.id).await.unwrap();
        assert_eq!(user.id, get_user.id);

        println!("testing update user ");
        get_user.name = "1111".to_string();
        let update_user = sut.update(&get_user).await.unwrap();
        assert_eq!("1111", &update_user.name);
        // info!("{}", serde_json::to_string(&update_user).unwrap());

        println!("testing list users ");
        let user_option = UserOption {
            name: Some(String::from("1111")),
            ..Default::default()
        };
        let users = sut.list(user_option).await.unwrap();
        assert_eq!(1, users.len());
        // info!("{}", serde_json::to_string(users).unwrap());

        info!("testing delete user ");
        let old_user = users.get(0).unwrap();
        let delete_user = sut.delete(old_user.id).await.unwrap();

        assert_eq!(
            serde_json::to_string(old_user).unwrap(),
            serde_json::to_string(&delete_user).unwrap()
        );

        // let users = sut.list(&user_option).await.unwrap();
        // assert_eq!(0, users.len());

        Ok(())
    }
}
