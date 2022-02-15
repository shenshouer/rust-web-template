use async_trait::async_trait;
use std::{fmt::Display, sync::Arc};

use super::PgPool;
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// User创建参数
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub mobile: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub mobile: String,
}

// list 查询条件
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserOption {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl UserOption {
    pub fn new_user(self, origin_user: User) -> User {
        let mut user = User { ..origin_user };
        if let Some(username) = self.username {
            user.username = username
        }

        if let Some(first_name) = self.first_name {
            user.first_name = first_name
        }

        if let Some(last_name) = self.last_name {
            user.last_name = last_name
        }

        if let Some(email) = self.email {
            user.email = email
        }

        if let Some(mobile) = self.mobile {
            user.mobile = mobile
        }

        user
    }
}

// 实现std::fmt::Display trait，方便在format!中组装sql的查询条件
impl Display for UserOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // where condition assemble
        let mut where_condition = String::from("");
        if let Some(ref username) = self.username {
            where_condition = format!("username='{username}' AND");
        }
        if let Some(ref first_name) = self.first_name {
            where_condition = format!("{where_condition} first_name='{first_name}' AND");
        }
        if let Some(ref last_name) = self.last_name {
            where_condition = format!("{where_condition} last_name='{last_name}' AND");
        }
        if let Some(ref email) = self.email {
            where_condition = format!("{where_condition} email='{email}' AND");
        }
        if let Some(ref mobile) = self.mobile {
            where_condition = format!("{where_condition} mobile='{mobile}'");
        }

        if where_condition.len() > 0 {
            where_condition = format!("WHERE {where_condition}");
            if where_condition.ends_with("AND") {
                where_condition = where_condition.strip_suffix("AND").unwrap().into();
            }
        }

        let mut offset_condition;
        if let Some(offset) = self.offset {
            offset_condition = format!("OFFSET {offset}");
        } else {
            offset_condition = format!("OFFSET 0");
        }
        if let Some(limit) = self.limit {
            offset_condition = format!("{offset_condition} LIMIT {limit}");
        } else {
            offset_condition = format!("{offset_condition} LIMIT 20");
        }
        write!(f, " {where_condition} {offset_condition}")
    }
}

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
        let sql = "INSERT INTO users (username, first_name, last_name, email, mobile) VALUES ($1, $2, $3, $4, $5) RETURNING *;";
        let user = sqlx::query_as::<_, User>(sql)
            .bind(&user.username)
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
            username = $1, 
            first_name = $2, 
            last_name = $3,  
            email = $4,
            mobile = $5
            WHERE id = $6
            RETURNING *
            "#;
        let user = sqlx::query_as::<_, User>(sql)
            .bind(&user.username)
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
    // use tracing_test::traced_test;

    #[test]
    fn test_user_option_as_sql_condition() {
        use super::UserOption;

        let default_option = UserOption::default();
        let expect_offset_condition = String::from("  OFFSET 0 LIMIT 20");
        let offset_condition = format!("{default_option}");
        assert_eq!(expect_offset_condition, offset_condition);

        let where_option_one = UserOption {
            username: Some("username".to_string()),
            mobile: Some("18612424366".to_string()),
            offset: Some(2),
            ..Default::default()
        };
        let expect_condition =
            " WHERE username='username' AND mobile='18612424366' OFFSET 2 LIMIT 20";
        let condition = format!("{where_option_one}");
        assert_eq!(expect_condition, condition);

        let where_option_two = UserOption {
            mobile: Some("18612424366".to_string()),
            offset: Some(4),
            ..Default::default()
        };
        let expect_condition = " WHERE  mobile='18612424366' OFFSET 4 LIMIT 20";
        let condition = format!("{where_option_two}");
        assert_eq!(expect_condition, condition);
    }

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
            username: "u1".to_string(),
            first_name: "fn1".to_string(),
            last_name: "ln1".to_string(),
            email: "email1".to_string(),
            mobile: "18612424366".to_string(),
        };

        info!("testing create new user ");
        let ref user = sut.create(&create_entity).await.unwrap();

        assert_eq!(user.username, create_entity.username);
        assert_eq!(false, user.id.is_nil());

        info!("testing get user ");
        let mut get_user = sut.get(user.id).await.unwrap();
        assert_eq!(user.id, get_user.id);

        println!("testing update user ");
        get_user.username = "uu1".to_string();
        let update_user = sut.update(&get_user).await.unwrap();
        assert_eq!("uu1", &update_user.username);
        // info!("{}", serde_json::to_string(&update_user).unwrap());

        println!("testing list users ");
        let user_option = UserOption {
            username: Some(String::from("uu1")),
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
