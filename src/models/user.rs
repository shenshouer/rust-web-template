use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

// User创建参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub const TABLE: &'static str = "users";
}

impl Default for User {
    fn default() -> Self {
        User {
            id: Uuid::new_v4(),
            name: Default::default(),
            email: Default::default(),
            password: Default::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

// list 查询条件
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserOption {
    pub name: Option<String>,
    pub email: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

// impl UserOption {
//     pub fn new_user(self, origin_user: User) -> User {
//         let mut user = User { ..origin_user };

//         if let Some(name) = self.name {
//             user.name = name
//         }

//         if let Some(email) = self.email {
//             user.email = email
//         }

//         user
//     }
// }

// 实现std::fmt::Display trait，方便在format!中组装sql的查询条件
impl Display for UserOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // where condition assemble
        let mut where_condition = String::from("");
        if let Some(ref name) = self.name {
            where_condition = format!("{where_condition}name='{name}' AND ");
        }

        if let Some(ref email) = self.email {
            where_condition = format!("{where_condition}email='{email}' AND ");
        }

        if where_condition.len() > 0 {
            where_condition = format!("WHERE {where_condition}").trim_end().to_string();
            if where_condition.ends_with("AND") {
                where_condition = where_condition.strip_suffix("AND").unwrap().into();
            }
        }

        let mut offset_condition;
        if let Some(offset) = self.offset {
            offset_condition = format!("OFFSET {offset}");
        } else {
            offset_condition = format!("OFFSET 0");
        }
        if let Some(limit) = self.limit {
            offset_condition = format!("{offset_condition} LIMIT {limit}");
        } else {
            offset_condition = format!("{offset_condition} LIMIT 20");
        }
        write!(f, "{where_condition}{offset_condition}")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_user_option_as_sql_condition() {
        use super::UserOption;

        let default_option = UserOption::default();
        let expect_offset_condition = String::from("OFFSET 0 LIMIT 20");
        let offset_condition = format!("{default_option}");
        assert_eq!(expect_offset_condition, offset_condition);

        let where_option_one = UserOption {
            name: Some("18612424366".to_string()),
            offset: Some(2),
            ..Default::default()
        };
        let expect_condition = "WHERE name='18612424366' OFFSET 2 LIMIT 20";
        let condition = format!("{where_option_one}");
        assert_eq!(expect_condition, condition);

        let where_option_two = UserOption {
            name: Some("18612424366".to_string()),
            offset: Some(4),
            ..Default::default()
        };
        let expect_condition = "WHERE name='18612424366' OFFSET 4 LIMIT 20";
        let condition = format!("{where_option_two}");
        assert_eq!(expect_condition, condition);
    }
}
