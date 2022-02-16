use crate::{
    errors::AppError,
    services::user::{CreateUser, User, UserOption, UserService, UserServiceImpl},
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
    let user_svc = UserServiceImpl::new(pool);
    configure(user_svc)
}

fn configure<T>(user_svc: T) -> Router
where
    T: UserService + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(create_user::<T>).get(list_user::<T>))
        .route(
            "/:user_id",
            get(get_user::<T>)
                .delete(delete_user::<T>)
                .put(update_user::<T>),
        )
        .layer(&AddExtensionLayer::new(user_svc))
}

async fn create_user<T: UserService>(
    Extension(svc): Extension<T>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    Ok(svc.create(&payload).await?.into())
}

async fn get_user<T: UserService>(
    Extension(svc): Extension<T>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    Ok(svc.get(id).await?.into())
}

async fn delete_user<T: UserService>(
    Extension(svc): Extension<T>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    Ok(svc.delete(id).await?.into())
}

async fn update_user<T: UserService>(
    Extension(svc): Extension<T>,
    Path(id): Path<Uuid>,
    Json(opt): Json<UserOption>,
) -> Result<Json<User>, AppError> {
    Ok(svc.update(id, &opt).await?.into())
}

async fn list_user<T: UserService>(
    Extension(svc): Extension<T>,
    Query(mut opt): Query<UserOption>,
) -> Result<Json<Vec<User>>, AppError> {
    match opt.limit {
        None => opt.limit = Some(20),
        Some(n) if n > 100 => opt.limit = Some(100),
        Some(n) => opt.limit = Some(n),
    }

    match opt.offset {
        None => opt.offset = Some(0),
        Some(n) => opt.offset = Some(n),
    }
    Ok(svc.list(&opt).await?.into())
}

#[cfg(test)]
mod tests {
    /// 单元测试不全面，mockall实现的trait + Clone 在axum中传递有问题
    use super::*;
    use crate::services::user::{CreateUser, User, UserService};
    use async_trait::async_trait;
    use axum::{
        body::Body,
        http::{self, request::Request, StatusCode},
    };
    use tower::ServiceExt;

    #[derive(Clone)]
    struct MockUserService {}
    #[async_trait]
    impl UserService for MockUserService {
        async fn create(&self, user: &CreateUser) -> Result<User, AppError> {
            Ok(User {
                username: user.username.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                email: user.email.clone(),
                mobile: user.mobile.clone(),
                ..Default::default()
            })
        }

        async fn get(&self, id: Uuid) -> Result<User, AppError> {
            Ok(User {
                id: id,
                ..Default::default()
            })
        }
        async fn delete(&self, id: Uuid) -> Result<User, AppError> {
            Ok(User {
                id: id,
                ..Default::default()
            })
        }
        async fn update(&self, id: Uuid, opt: &UserOption) -> Result<User, AppError> {
            let user = User {
                id: id,
                ..Default::default()
            };
            let user = opt.clone().new_user(user);
            Ok(user)
        }
        async fn list(&self, _opt: &UserOption) -> Result<Vec<User>, AppError> {
            Ok(Vec::new())
        }
    }

    impl MockUserService {
        fn new() -> Self {
            MockUserService {}
        }
    }

    #[tokio::test]
    async fn test_user_controller_create() {
        let create_user_param = &CreateUser {
            username: "u".to_string(),
            first_name: "fn".to_string(),
            last_name: "ln".to_string(),
            email: "shenshouer51@gmail.com".to_string(),
            mobile: "18612424366".to_string(),
        };

        let app = configure(MockUserService::new());

        // create api test
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(create_user_param).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let user: User = serde_json::from_slice(&body).unwrap();
        assert_eq!(create_user_param.mobile, user.mobile);
    }

    #[tokio::test]
    async fn test_user_controller_get() {
        let app = configure(MockUserService::new());

        let uid = Uuid::new_v4();
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/{}", uid))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let user: User = serde_json::from_slice(&body).unwrap();
        assert_eq!(uid, user.id);
    }
}
