/// 认证模块
///
use crate::{
    errors::AppError,
    services::auth::{AuthService, AuthServiceImpl, Credential, Token},
};
use axum::{
    extract::Extension, response::IntoResponse, routing::post, AddExtensionLayer, Json, Router,
};
use sqlx::postgres::PgPool;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct TokenParam {
    pub token: Token,
}

pub(crate) fn router(pool: Arc<PgPool>, client: Arc<redis::Client>) -> Router {
    configure(AuthServiceImpl::new(pool, client))
}

fn configure<T>(svc: T) -> Router
where
    T: AuthService + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/register", post(registry::<T>))
        .route("/login", post(login::<T>))
        .route("/authenticate", post(authenticate::<T>))
        .layer(&AddExtensionLayer::new(svc))
}

async fn registry<T: AuthService>(
    Extension(svc): Extension<T>,
    Json(payload): Json<Credential>,
) -> Result<Json<bool>, AppError> {
    Ok(svc.register(&payload).await?.into())
}

async fn login<T: AuthService>(
    Extension(svc): Extension<T>,
    Json(payload): Json<Credential>,
) -> impl IntoResponse {
    Json(svc.login(&payload).await)
}

async fn authenticate<T: AuthService>(
    Extension(svc): Extension<T>,
    Json(token): Json<TokenParam>,
) -> impl IntoResponse {
    Json(svc.authenticate(&token.token).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{errors::AppError, services::auth::AuthService};
    use async_trait::async_trait;
    use axum::{
        body::Body,
        http::{self, request::Request, StatusCode},
    };
    use tower::ServiceExt;

    #[derive(Clone, Default)]
    struct MockAuthService {
        pub failure_register: Option<bool>,
        pub failure_login: Option<bool>,
        pub failure_authenicate: Option<bool>,
    }
    impl MockAuthService {
        fn new() -> Self {
            MockAuthService::default()
        }
    }
    #[async_trait]
    impl AuthService for MockAuthService {
        async fn register(&self, _credential: &Credential) -> Result<bool, AppError> {
            if let Some(ok) = self.failure_register {
                if ok {
                    return Err(AppError::new_other_error(
                        "failure with register".to_string(),
                    ));
                }
            }
            Ok(true)
        }
        async fn login(&self, _credential: &Credential) -> Option<Token> {
            if let Some(ok) = self.failure_login {
                if ok {
                    return None;
                }
            }
            Some("mock_test".to_string())
        }
        async fn authenticate(&self, _token: &Token) -> Option<String> {
            if let Some(ok) = self.failure_authenicate {
                if ok {
                    return None;
                }
            }
            Some("mock_test".to_string())
        }
    }

    #[tokio::test]
    async fn test_authenticate_failure() {
        let mut svc = MockAuthService::new();

        svc.failure_authenicate = Some(true);

        let app = configure(svc);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/authenticate")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&TokenParam {
                            token: "test".to_string(),
                        })
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let actual: Option<String> = serde_json::from_slice(&body).unwrap();
        assert_eq!(actual, None);
    }

    #[tokio::test]
    async fn test_authenticate_success() {
        let svc = MockAuthService::new();
        let app = configure(svc);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/authenticate")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&TokenParam {
                            token: "test".to_string(),
                        })
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let actual: Option<String> = serde_json::from_slice(&body).unwrap();
        assert_eq!(actual, Some("mock_test".to_string()));
    }
}
