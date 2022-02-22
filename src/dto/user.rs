use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 4, max = 10))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6), must_match = "password2")]
    pub password: String,
    #[validate(length(min = 6))]
    pub password2: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserInput {
    #[validate(length(min = 4, max = 10))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 6), must_match = "password2")]
    pub password: Option<String>,
    #[validate(length(min = 6))]
    pub password2: Option<String>,
}

impl UpdateUserInput {
    pub fn check(&self) -> bool {
        if self.name.is_none() && self.email.is_none() && self.password.is_none() {
            return false;
        }
        true
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct ListUserInput {
    #[validate(length(min = 4, max = 10))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[serde(flatten)]
    pub limit_offset: PagationInput,
}

/// 翻页组件
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PagationInput {
    #[validate(custom = "validate_pagation_limit")]
    pub limit: Option<u32>,
    #[validate(range(min = 0))]
    pub offset: Option<u32>,
}

impl Default for PagationInput {
    fn default() -> Self {
        PagationInput {
            limit: Some(20),
            offset: Some(0),
        }
    }
}

impl PagationInput {
    pub fn check(&mut self) {
        if let Some(size) = self.limit {
            if size > 100 {
                self.limit = Some(100);
            }
        }

        if self.offset.is_none() {
            self.offset = Some(0);
        }
    }
}

const PAGATION_LIMITS: [u32; 4] = [10, 20, 50, 100];
fn validate_pagation_limit(limit: u32) -> Result<(), ValidationError> {
    if !PAGATION_LIMITS.contains(&limit) {
        // let msg = format!("value must be in {:?}", PAGATION_LIMITS);
        return Err(ValidationError::new("value must be in [10, 20, 50, 100]"));
    }

    Ok(())
}
