use axum::async_trait;
use std::sync::Arc;

use crate::{
    errors::AppError,
    models::user::{CreateUser, User, UserOption},
};
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepo {
    async fn create(&self, user: &CreateUser) -> Result<User, AppError>;
    async fn get(&self, id: Uuid) -> Result<User, AppError>;
    async fn delete(&self, id: Uuid) -> Result<User, AppError>;
    async fn update(&self, user: &User) -> Result<User, AppError>;
    async fn list(&self, fields: &UserOption) -> Result<Vec<User>, AppError>;
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
    async fn create(&self, user: &CreateUser) -> Result<User, AppError> {
        let sql = "INSERT INTO users (first_name, last_name, email, mobile) VALUES ($1, $2, $3, $4) RETURNING *;";
        let user = sqlx::query_as::<_, User>(sql)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.email)
            .bind(&user.mobile)
            .fetch_one(&*self.pool)
            .await?;
        Ok(user)
    }

    async fn get(&self, id: Uuid) -> Result<User, AppError> {
        let sql = "SELECT * FROM users WHERE id = $1";
        let user = sqlx::query_as::<_, User>(sql)
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<User, AppError> {
        let user = sqlx::query_as(
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await?;
        Ok(user)
    }

    async fn update(&self, user: &User) -> Result<User, AppError> {
        let sql = r#"
        UPDATE users SET 
            first_name = $1, 
            last_name = $2,  
            email = $3,
            mobile = $4
            WHERE id = $5
            RETURNING *
            "#;
        let user = sqlx::query_as::<_, User>(sql)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.email)
            .bind(&user.mobile)
            .bind(&user.id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(user)
    }

    async fn list(&self, opts: &UserOption) -> Result<Vec<User>, AppError> {
        let sql = format!("SELECT * FROM users {opts}");
        let rows = sqlx::query_as(&sql).fetch_all(&*self.pool).await?;
        Ok(rows)
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

        // test create entity
        let create_entity = CreateUser {
            first_name: "fn1".to_string(),
            last_name: "ln1".to_string(),
            email: "email1".to_string(),
            mobile: "18612424366".to_string(),
        };

        info!("testing create new user ");
        let ref user = sut.create(&create_entity).await.unwrap();

        assert_eq!(user.mobile, create_entity.mobile);
        assert_eq!(false, user.id.is_nil());

        info!("testing get user ");
        let mut get_user = sut.get(user.id).await.unwrap();
        assert_eq!(user.id, get_user.id);

        println!("testing update user ");
        get_user.mobile = "1111".to_string();
        let update_user = sut.update(&get_user).await.unwrap();
        assert_eq!("1111", &update_user.mobile);
        // info!("{}", serde_json::to_string(&update_user).unwrap());

        println!("testing list users ");
        let user_option = UserOption {
            first_name: Some(String::from("fn1")),
            ..Default::default()
        };
        let ref users = sut.list(&user_option).await.unwrap();
        assert_eq!(1, users.len());
        // info!("{}", serde_json::to_string(users).unwrap());

        info!("testing delete user ");
        let old_user = users.get(0).unwrap();
        let ref delete_user = sut.delete(old_user.id).await.unwrap();

        assert_eq!(
            serde_json::to_string(old_user).unwrap(),
            serde_json::to_string(delete_user).unwrap()
        );

        let users = sut.list(&user_option).await.unwrap();
        assert_eq!(0, users.len());

        Ok(())
    }
}