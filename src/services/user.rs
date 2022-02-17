pub use crate::{
    dao::user_repo::{UserRepo, UserRepoImpl},
    errors::AppError,
    models::user::{CreateUser, User, UserOption},
};
use axum::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

// #[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserService {
    async fn create(&self, user: &CreateUser) -> Result<User, AppError>;
    async fn get(&self, id: Uuid) -> Result<User, AppError>;
    async fn delete(&self, id: Uuid) -> Result<User, AppError>;
    async fn update(&self, id: Uuid, opt: &UserOption) -> Result<User, AppError>;
    async fn list(&self, opt: &UserOption) -> Result<Vec<User>, AppError>;
}

#[derive(Clone)]
pub struct UserServiceImpl<T: UserRepo>
where
    T: UserRepo + Sync + Send,
{
    pub user_repo: T,
}

impl UserServiceImpl<UserRepoImpl> {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserServiceImpl {
            user_repo: UserRepoImpl::new(pool),
        }
    }
}

#[async_trait]
impl<T> UserService for UserServiceImpl<T>
where
    T: UserRepo + Sync + Send,
{
    async fn create(&self, user: &CreateUser) -> Result<User, AppError> {
        Ok(self.user_repo.create(user).await?)
    }

    async fn get(&self, id: Uuid) -> Result<User, AppError> {
        Ok(self.user_repo.get(id).await?)
    }

    async fn delete(&self, id: Uuid) -> Result<User, AppError> {
        Ok(self.user_repo.delete(id).await?)
    }

    async fn update(&self, id: Uuid, opt: &UserOption) -> Result<User, AppError> {
        let origin_user = self.user_repo.get(id).await?;
        let user = opt.clone().new_user(origin_user);
        Ok(self.user_repo.update(&user).await?)
    }

    async fn list(&self, opt: &UserOption) -> Result<Vec<User>, AppError> {
        Ok(self.user_repo.list(opt).await?)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        dao::user_repo::MockUserRepo,
        models::user::{CreateUser, User, UserOption},
    };
    use chrono::Utc;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_user_service_create() {
        let mut user_repo = MockUserRepo::new();

        user_repo
            .expect_create()
            .with(always())
            .times(1)
            .returning(|param| {
                Ok(User {
                    id: Uuid::new_v4(),
                    name: param.name.clone(),
                    password: param.password.clone(),
                    email: param.email.clone(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                })
            });

        let sut = UserServiceImpl { user_repo };
        let mock_create_result = sut
            .create(&CreateUser {
                name: "f1".to_string(),
                password: "l1".to_string(),
                email: "shenshouer51@gmail.com".to_string(),
            })
            .await;
        assert!(mock_create_result.is_ok());
    }

    #[tokio::test]
    async fn test_user_service_get() {
        let mut user_repo = MockUserRepo::new();
        let get_param = Uuid::new_v4();
        user_repo
            .expect_get()
            .with(eq(get_param.clone()))
            .returning(|id| {
                Ok(User {
                    id: id,
                    ..Default::default()
                })
            });

        let sut = UserServiceImpl { user_repo };

        let mock_get_result = sut.get(get_param).await;
        assert!(mock_get_result.is_ok());
    }

    #[tokio::test]
    async fn test_user_service_delete() {
        let mut user_repo = MockUserRepo::new();
        let delete_param = Uuid::new_v4();
        user_repo
            .expect_delete()
            .with(eq(delete_param.clone()))
            .returning(|id| {
                Ok(User {
                    id: id,
                    ..Default::default()
                })
            });
        let sut = UserServiceImpl { user_repo };
        let mock_delete_result = sut.delete(delete_param).await;
        assert!(mock_delete_result.is_ok());
    }

    #[tokio::test]
    async fn test_user_service_list() {
        let mut user_repo = MockUserRepo::new();
        user_repo
            .expect_list()
            .with(always())
            .returning(|_x| Ok(Vec::new()));
        let sut = UserServiceImpl { user_repo };
        let mock_list_result = sut.list(&UserOption::default()).await;
        assert!(mock_list_result.is_ok());
    }

    #[tokio::test]
    async fn test_user_service_update() {
        let mut user_repo = MockUserRepo::new();
        let uid = Uuid::new_v4();
        user_repo
            .expect_get()
            .with(eq(uid.clone()))
            .returning(|id| {
                Ok(User {
                    id: id,
                    ..Default::default()
                })
            });

        user_repo
            .expect_update()
            .with(always())
            .returning(|user| Ok(user.clone()));

        let sut = UserServiceImpl { user_repo };

        let opt = &UserOption {
            name: Some("fk".to_string()),
            ..Default::default()
        };
        let mock_update_result = sut.update(uid, opt).await;
        assert!(mock_update_result.is_ok());
    }

    #[tokio::test]
    async fn test_user_repo_update() {
        let mut user_repo = MockUserRepo::new();
        user_repo
            .expect_update()
            .with(always())
            .returning(|user| Ok(user.clone()));

        let result = user_repo.update(&User::default()).await;
        assert!(result.is_ok());
    }
}
