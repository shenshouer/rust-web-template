/// 认证实现
mod auth;
/// 主页
mod home;
/// token相关功能
mod jwt;
/// 用户模块逻辑层
mod users;

use super::{
    errors::{ApiError, Error},
    models::user::User,
    services::auth::{AuthServiceImpl, DynAuthService},
};
use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    http::StatusCode,
    response::{IntoResponse, Response},
    AddExtensionLayer, Json, Router,
};
use headers::{authorization::Bearer, Authorization};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::sync::Arc;

// 路由配置
pub fn routers(pool: Arc<PgPool>) -> Router {
    let auth_svc: DynAuthService = Arc::new(AuthServiceImpl::new(pool.clone()));
    Router::new()
        .nest("/users", users::router(pool))
        .nest("/auth", auth::router())
        .nest("", home::router())
        .layer(&AddExtensionLayer::new(auth_svc))
}

// 统一APi成功响应格式
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    // 直接生成成功相应数据结构
    fn success(data: T) -> ApiResponse<T> {
        ApiResponse {
            ok: true,
            data: Some(data),
            error: None,
        }
    }
}

// 从请求中直接获取到用户信息
#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(Error::from)?;
        let Extension(svc) = Extension::<DynAuthService>::from_request(req)
            .await
            .map_err(Error::from)?;

        let claims = jwt::verify(bearer.token())?;
        let user = svc.get(claims.sub).await?;
        Ok(user)
    }
}
