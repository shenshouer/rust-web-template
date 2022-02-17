use crate::{errors::AppError, models::credential::Credential};
use axum::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct CredentialRepoImpl {
    pool: Arc<PgPool>,
}

impl CredentialRepoImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        CredentialRepoImpl { pool }
    }
}

use super::CredentialRepo;

#[async_trait]
impl CredentialRepo for CredentialRepoImpl {
    async fn save_credential(&self, credential: &Credential) -> Result<bool, AppError> {
        let result = sqlx::query(
            "insert into credentials (username, password) values ($1, crypt($2, gen_salt('bf')))",
        )
        .bind(&credential.username)
        .bind(&credential.password)
        .execute(&*self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn is_credential_exists(&self, credential: &Credential) -> bool {
        let (found,): (bool,) = sqlx::query_as(
            "select true from credentials where username = $1 and password = crypt($2, password)",
        )
        .bind(&credential.username)
        .bind(&credential.password)
        .fetch_one(&*self.pool)
        .await
        .unwrap_or((false,));
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_save_and_check_credential_repo() {
        tracing_subscriber::fmt::init();

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:p%40ssword%21@localhost")
            .await
            .expect("Unable to connect to DB");
        sqlx::query("drop database if exists test_credential_repo")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("create database test_credential_repo")
            .execute(&pool)
            .await
            .unwrap();

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:p%40ssword%21@localhost/test_credential_repo")
            .await
            .expect("Unable to connect to test_credential_repo DB");

        sqlx::migrate!().run(&pool).await.unwrap();

        let sut = CredentialRepoImpl {
            pool: Arc::new(pool),
        };

        let credential = Credential {
            username: "u".to_string(),
            password: "p".to_string(),
        };

        assert_eq!(false, sut.is_credential_exists(&credential).await);
        assert!(sut.save_credential(&credential).await.is_ok());
        assert_eq!(true, sut.is_credential_exists(&credential).await);
    }
}
