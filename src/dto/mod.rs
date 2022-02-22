pub(crate) mod auth;
pub(crate) mod user;

use validator::Validate;

use crate::errors::Result;

/// 输入验证与检查
pub fn validate_payload<T: Validate>(payload: &T) -> Result<()> {
    Ok(payload.validate()?)
}
