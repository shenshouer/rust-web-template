use super::ApiResponse;
use crate::{
    dto::validate_payload,
    errors::{ApiResult, Error},
    services::user::{
        DynUserService, ListUserInput, RegisterInput, UpdateUserInput, User, UserServiceImpl,
    },
};
use axum::{
    extract::{Extension, Path, Query},
    routing::{get, post},
    AddExtensionLayer, Json, Router,
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) fn router(pool: Arc<PgPool>) -> Router {
    let user_svc: DynUserService = Arc::new(UserServiceImpl::new(pool));
    configure(user_svc)
}

fn configure(user_svc: DynUserService) -> Router {
    Router::new()
        .route("/", post(create_user).get(list_user))
        .route(
            "/:user_id",
            get(get_user).delete(delete_user).put(update_user),
        )
        .layer(&AddExtensionLayer::new(user_svc))
}

async fn create_user(
    Extension(svc): Extension<DynUserService>,
    Json(input): Json<RegisterInput>,
) -> ApiResult<ApiResponse<User>> {
    validate_payload(&input)?;
    Ok(ApiResponse::success(svc.create(input).await?))
}

async fn get_user(
    Extension(svc): Extension<DynUserService>,
    Path(id): Path<Uuid>,
) -> ApiResult<ApiResponse<User>> {
    // Ok(svc.get(id).await?.into())
    Ok(ApiResponse::success(svc.get(id).await?))
}

async fn delete_user(
    Extension(svc): Extension<DynUserService>,
    Path(id): Path<Uuid>,
) -> ApiResult<ApiResponse<User>> {
    Ok(ApiResponse::success(svc.delete(id).await?))
}

async fn update_user(
    Extension(svc): Extension<DynUserService>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateUserInput>,
) -> ApiResult<ApiResponse<User>> {
    if !input.check() {
        return Err(Error::new_empty_fields_error("update user failed".to_string()).into());
    }
    validate_payload(&input)?;
    Ok(ApiResponse::success(svc.update(id, input).await?))
}

async fn list_user(
    Extension(svc): Extension<DynUserService>,
    Query(mut input): Query<ListUserInput>,
) -> ApiResult<ApiResponse<Vec<User>>> {
    input.limit_offset.check();
    validate_payload(&input)?;
    Ok(ApiResponse::success(svc.list(input).await?))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{routers::jwt, services::user::MockUserService};
    use axum::{
        body::Body,
        http::{self, request::Request, StatusCode},
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_user_controller_create() {
        let mut svc = MockUserService::new();
        svc.expect_create().returning(|input| {
            Ok(User {
                id: Uuid::new_v4(),
                name: input.name,
                email: input.email,
                password: input.password,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });

        let app = configure(Arc::new(svc));

        let create_user_param = &RegisterInput {
            name: "testname".to_string(),
            email: "shenshouer51@gmail.com".to_string(),
            password: "18612424366".to_string(),
            password2: "18612424366".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", jwt::sign(uuid::Uuid::new_v4()).unwrap()),
                    )
                    .body(Body::from(
                        serde_json::to_string(create_user_param).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let user: ApiResponse<User> = serde_json::from_slice(&body).unwrap();
        assert_eq!(create_user_param.name, user.data.unwrap().name);
    }

    #[tokio::test]
    async fn test_user_controller_get() {
        let mut svc = MockUserService::new();
        svc.expect_get().returning(|id| {
            Ok(User {
                id,
                ..Default::default()
            })
        });
        let app = configure(Arc::new(svc));

        let uid = Uuid::new_v4();
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/{}", uid))
                    .header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", jwt::sign(uuid::Uuid::new_v4()).unwrap()),
                    )
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let user: ApiResponse<User> = serde_json::from_slice(&body).unwrap();
        assert_eq!(uid, user.data.unwrap().id);
    }
}
