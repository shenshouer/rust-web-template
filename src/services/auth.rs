pub(crate) use crate::{
    dao::user_repo::{UserRepo, UserRepoImpl},
    dto::auth::LoginInput,
    errors::Result,
    models::{auth::Credential, user::User},
};
use axum::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub type DynAuthService = Arc<dyn AuthService + Send + Sync>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuthService {
    async fn sign_in(&self, input: LoginInput) -> Result<User>;
    async fn get(&self, id: Uuid) -> Result<User>;
}

#[derive(Clone)]
pub struct AuthServiceImpl<T: UserRepo>
where
    T: UserRepo + Sync + Send,
{
    pub user_repo: T,
}

impl AuthServiceImpl<UserRepoImpl> {
    pub fn new(pool: Arc<PgPool>) -> Self {
        AuthServiceImpl {
            user_repo: UserRepoImpl::new(pool),
        }
    }
}

#[async_trait]
impl<T> AuthService for AuthServiceImpl<T>
where
    T: UserRepo + Sync + Send,
{
    async fn sign_in(&self, input: LoginInput) -> Result<User> {
        let credential = Credential {
            email: input.email,
            password: input.password,
        };
        self.user_repo.authenticate(credential).await
    }

    async fn get(&self, id: Uuid) -> Result<User> {
        Ok(self.user_repo.get(id).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dao::user_repo::MockUserRepo, errors::Error, models::user::User};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_login_success() {
        let mut user_repo = MockUserRepo::new();
        user_repo
            .expect_authenticate()
            .with(always())
            .returning(|_| Ok(User::default()));
        let sut = AuthServiceImpl { user_repo };

        let input = LoginInput {
            email: "shenshouer51@163.com".to_string(),
            password: "123456".to_string(),
        };
        let actual = sut.sign_in(input).await;
        assert!(actual.is_ok());
    }

    #[tokio::test]
    async fn test_login_failure() {
        let input = LoginInput {
            email: "u".to_string(),
            password: "p".to_string(),
        };
        let mut user_repo = MockUserRepo::new();
        user_repo
            .expect_authenticate()
            .with(always())
            .returning(|_| {
                Err(Error::new_empty_fields_error(
                    "mock login failure in auth service".to_string(),
                ))
            });
        let sut = AuthServiceImpl { user_repo };

        let actual = sut.sign_in(input).await;
        assert!(actual.is_err());
    }
}
