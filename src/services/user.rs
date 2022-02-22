pub(crate) use crate::{
    dao::user_repo::{UserRepo, UserRepoImpl},
    dto::user::{ListUserInput, RegisterInput, UpdateUserInput},
    errors::{Error, Result},
    models::user::{CreateUser, User, UserOption},
};
use axum::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub type DynUserService = Arc<dyn UserService + Send + Sync>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserService {
    async fn create(&self, input: RegisterInput) -> Result<User>;
    async fn get(&self, id: Uuid) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<User>;
    async fn update(&self, id: Uuid, opt: UpdateUserInput) -> Result<User>;
    async fn list(&self, input: ListUserInput) -> Result<Vec<User>>;
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
    async fn create(&self, input: RegisterInput) -> Result<User> {
        let user = CreateUser {
            name: input.name,
            email: input.email,
            password: input.password,
        };

        let email = user.email.clone();
        if self.user_repo.get_by_email(&email).await.is_ok() {
            return Err(Error::DuplicateUserEmail(email));
        }

        Ok(self.user_repo.create(user).await?)
    }

    async fn get(&self, id: Uuid) -> Result<User> {
        Ok(self.user_repo.get(id).await?)
    }

    async fn delete(&self, id: Uuid) -> Result<User> {
        Ok(self.user_repo.delete(id).await?)
    }

    async fn update(&self, id: Uuid, input: UpdateUserInput) -> Result<User> {
        let mut origin_user = self.user_repo.get(id).await?;

        if let Some(name) = input.name {
            origin_user.name = name;
        }

        if let Some(email) = input.email {
            origin_user.email = email;
        }

        if let Some(password) = input.password {
            origin_user.password = password;
        }
        Ok(self.user_repo.update(&origin_user).await?)
    }

    async fn list(&self, input: ListUserInput) -> Result<Vec<User>> {
        let opt = UserOption {
            name: input.name,
            email: input.email,
            offset: input.limit_offset.offset,
            limit: input.limit_offset.limit,
        };
        Ok(self.user_repo.list(opt).await?)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{dao::user_repo::MockUserRepo, models::user::User};
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
            .create(RegisterInput {
                name: "f1".to_string(),
                password: "l1".to_string(),
                password2: "l1".to_string(),
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
        let mock_list_result = sut.list(ListUserInput::default()).await;
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

        let opt = UpdateUserInput {
            name: Some("fk".to_string()),
            email: None,
            password: None,
            password2: None,
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
