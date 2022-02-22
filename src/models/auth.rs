use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Credential {
    pub email: String,
    pub password: String,
}
