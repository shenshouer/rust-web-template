use super::jwt;
use super::ApiResponse;
use crate::{
    config::constants::BEARER,
    dto::{
        auth::{LoginInput, TokenPayload},
        validate_payload,
    },
    errors::{ApiResult, Error},
    services::auth::{DynAuthService, User},
};
use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};

pub(crate) fn router() -> Router {
    configure()
}

fn configure() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/authorize", get(authorize))
}

pub async fn authorize(user: User) -> ApiResponse<User> {
    ApiResponse::success(user)
}

async fn login(
    Extension(svc): Extension<DynAuthService>,
    Json(input): Json<LoginInput>,
) -> ApiResult<ApiResponse<TokenPayload>> {
    validate_payload(&input)?;
    let user = svc
        .sign_in(input)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    let token = jwt::sign(user.id)?;
    Ok(ApiResponse::success(TokenPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::auth::MockAuthService;
    use axum::AddExtensionLayer;
    use axum::{
        body::Body,
        http::{self, request::Request, StatusCode},
    };
    use mockall::predicate::*;
    use std::sync::Arc;
    use tower::ServiceExt;

    fn configure_with_auth_service(auth_svc: DynAuthService) -> Router {
        configure().layer(&AddExtensionLayer::new(auth_svc))
    }

    #[tokio::test]
    async fn test_login_success() {
        let mut svc = MockAuthService::new();
        svc.expect_sign_in()
            .with(always())
            .returning(|_input| Ok(User::default()));

        std::env::set_var("JWT_SECRET", "example_secret_key");
        let app = configure_with_auth_service(Arc::new(svc));

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/login")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&LoginInput {
                            email: "shenshouer@163.com".to_string(),
                            password: "password".to_string(),
                        })
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // println!("----------> {:?}", core::str::from_utf8(&body.to_owned()));
        let actual: ApiResponse<TokenPayload> = serde_json::from_slice(&body).unwrap();
        // println!("{:?}", actual);
        assert!(actual.ok);
        assert!(actual.data.is_some());
    }

    #[tokio::test]
    async fn test_authenticate_success() {
        let mut svc = MockAuthService::new();
        svc.expect_get().returning(|id| {
            Ok(User {
                id,
                email: "test@example.com".to_string(),
                ..Default::default()
            })
        });

        std::env::set_var("JWT_SECRET", "example_secret_key");
        let app = configure_with_auth_service(Arc::new(svc));

        let token = format!("Bearer {}", jwt::sign(uuid::Uuid::new_v4()).unwrap());
        println!("---->> token: {}", &token);
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/authorize")
                    .header(http::header::AUTHORIZATION, token)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let actual: ApiResponse<User> = serde_json::from_slice(&body).unwrap();
        assert!(actual.ok);
        assert_eq!(actual.data.unwrap().email, "test@example.com");
    }
}
