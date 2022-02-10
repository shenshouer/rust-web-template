use crate::models::user::{CreateUser, User, UserOption, UserRepo, UserRepoImpl};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
trait UserService {
    async fn create(&self, user: &CreateUser) -> Result<User>;
    async fn get(&self, id: Uuid) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<User>;
    async fn update(&self, user: User) -> Result<User>;
    async fn list(&self, fields: &UserOption) -> Result<Vec<User>>;
}

pub struct UserServiceImpl<T: UserRepo>
where
    T: UserRepo + Sync + Send,
{
    pub user_repo: T,
}

#[async_trait]
impl<T> UserService for UserServiceImpl<T>
where
    T: UserRepo + Sync + Send,
{
    async fn create(&self, user: &CreateUser) -> Result<User> {
        Ok(self.user_repo.create(user).await?)
    }

    async fn get(&self, id: Uuid) -> Result<User> {
        Ok(self.user_repo.get(id).await?)
    }

    async fn delete(&self, id: Uuid) -> Result<User> {
        Ok(self.user_repo.delete(id).await?)
    }

    async fn update(&self, user: User) -> Result<User> {
        Ok(self.user_repo.update(user).await?)
    }

    async fn list(&self, fields: &UserOption) -> Result<Vec<User>> {
        Ok(self.user_repo.list(fields).await?)
    }
}

#[cfg(test)]
mod tests {
    // use crate::models::user::MockUserRepo;

    use super::*;
    use crate::models::user::{CreateUser, MockUserRepo, User, UserOption};

    #[tokio::test]
    async fn test_user_service_ok() {
        use mockall::predicate::*;
        use uuid::Uuid;

        let mut user_repo = MockUserRepo::new();

        user_repo
            .expect_create()
            .with(always())
            .times(1)
            .returning(|param| {
                Ok(User {
                    id: Uuid::new_v4(),
                    username: param.username.clone(),
                    first_name: param.first_name.clone(),
                    last_name: param.last_name.clone(),
                    email: param.email.clone(),
                    mobile: param.mobile.clone(),
                })
            });

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

        user_repo
            .expect_update()
            .with(always())
            .returning(|param| Ok(param));

        user_repo
            .expect_list()
            .with(always())
            .returning(|_x| Ok(Vec::new()));

        let sut = UserServiceImpl { user_repo };

        let mock_create_result = sut
            .create(&CreateUser {
                username: "u1".to_string(),
                first_name: "f1".to_string(),
                last_name: "l1".to_string(),
                email: "shenshouer51@gmail.com".to_string(),
                mobile: "18612424366".to_string(),
            })
            .await;
        assert!(mock_create_result.is_ok());

        let mock_get_result = sut.get(get_param).await;
        assert!(mock_get_result.is_ok());

        let mock_delete_result = sut.delete(delete_param).await;
        assert!(mock_delete_result.is_ok());

        let mock_update_result = sut
            .update(User {
                id: Uuid::new_v4(),
                ..Default::default()
            })
            .await;
        assert!(mock_update_result.is_ok());

        let mock_list_result = sut.list(&UserOption::default()).await;
        assert!(mock_list_result.is_ok());
    }

    // #[tokio::test]
    // async fn test_user_service_err() {
    //     use anyhow::anyhow;
    //     use mockall::predicate::*;

    //     let mut user_repo = MockUserRepo::new();

    //     user_repo
    //         .expect_create()
    //         .with(never())
    //         .returning(|_param| Err(anyhow!("some error occured")));

    //     let sut = UserServiceImpl { user_repo };

    //     let mock_create_result = sut
    //         .create(&CreateUser {
    //             ..Default::default()
    //         })
    //         .await;
    //     assert!(mock_create_result.is_err());
    // }
}
