pub(crate) use crate::dao::{
    CredentialRepo, CredentialRepoImpl, RedisTokenRepoImpl, Token, TokenRepo,
};
use crate::errors::AppError;
pub(crate) use crate::models::credential::Credential;
use axum::async_trait;
use std::sync::Arc;
use tracing::error;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuthService {
    async fn register(&self, credential: &Credential) -> Result<bool, AppError>;
    async fn login(&self, credential: &Credential) -> Option<Token>;
    async fn authenticate(&self, token: &Token) -> Option<String>;
}

#[derive(Clone)]
pub struct AuthServiceImpl<A: CredentialRepo, B: TokenRepo> {
    pub credential_repo: A,
    pub token_repo: B,
}

impl AuthServiceImpl<CredentialRepoImpl, RedisTokenRepoImpl> {
    pub fn new(pool: Arc<sqlx::PgPool>, client: Arc<redis::Client>) -> Self {
        AuthServiceImpl {
            credential_repo: CredentialRepoImpl::new(pool),
            token_repo: RedisTokenRepoImpl::new(client),
        }
    }
}

#[async_trait]
impl<A, B> AuthService for AuthServiceImpl<A, B>
where
    A: CredentialRepo + Sync + Send,
    B: TokenRepo + Sync + Send,
{
    async fn register(self: &Self, credential: &Credential) -> Result<bool, AppError> {
        Ok(self.credential_repo.save_credential(credential).await?)
    }

    async fn login(self: &Self, credential: &Credential) -> Option<Token> {
        if !self.credential_repo.is_credential_exists(credential).await {
            return None;
        }

        let token = self.token_repo.generate_token().await;
        let result = self
            .token_repo
            .save_token(&token, &credential.username)
            .await;

        if let Err(err) = result {
            error!(
                "save token error: {} when login with username: {}",
                err, credential.username
            );
            return None;
        }
        Some(token)
    }

    async fn authenticate(self: &Self, token: &Token) -> Option<String> {
        self.token_repo.get_username_by_token(token).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::{MockCredentialRepo, MockTokenRepo};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_login_success() {
        let credential = Credential {
            username: "u".to_string(),
            password: "p".to_string(),
        };
        let token = "token".to_string();

        let mut credential_repo = MockCredentialRepo::new();
        credential_repo
            .expect_is_credential_exists()
            .with(eq(credential.clone()))
            .return_const(true);

        let mut token_repo = MockTokenRepo::new();
        token_repo
            .expect_generate_token()
            .return_const(token.clone());
        token_repo
            .expect_save_token()
            .with(eq(token.clone()), eq(credential.username.clone()))
            .returning(|_, _| Ok(()));

        let sut = AuthServiceImpl {
            credential_repo,
            token_repo,
        };

        let actual = sut.login(&credential).await;
        let expected = Some(token.clone());
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_login_failure_unable_to_save_token() {
        let credential = Credential {
            username: "u".to_string(),
            password: "p".to_string(),
        };
        let token = "token".to_string();

        let mut credential_repo = MockCredentialRepo::new();
        credential_repo
            .expect_is_credential_exists()
            .with(eq(credential.clone()))
            .return_const(true);

        let mut token_repo = MockTokenRepo::new();
        token_repo
            .expect_generate_token()
            .return_const(token.clone());
        token_repo
            .expect_save_token()
            .with(eq(token.clone()), eq(credential.username.clone()))
            .returning(|_, _| {
                Err(AppError::new_other_error(
                    "mock login failure and save token".to_string(),
                ))
            });

        let sut = AuthServiceImpl {
            credential_repo,
            token_repo,
        };

        let actual = sut.login(&credential).await;
        let expected = None;
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_login_failure_credential_does_not_exists() {
        let credential = Credential {
            username: "u".to_string(),
            password: "p".to_string(),
        };

        let mut credential_repo = MockCredentialRepo::new();
        credential_repo
            .expect_is_credential_exists()
            .with(eq(credential.clone()))
            .return_const(false);

        let token_repo = MockTokenRepo::new();

        let sut = AuthServiceImpl {
            credential_repo,
            token_repo,
        };

        let actual = sut.login(&credential).await;
        let expected = None;
        assert_eq!(expected, actual);
    }
}
