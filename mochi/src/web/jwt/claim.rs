use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::model;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Claims {
    pub iss: String,
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub sid: Uuid,
    pub preferred_username: String,
    pub email: String,
}

impl From<Claims> for model::User {
    fn from(Claims { sub, preferred_username, email, .. }: Claims) -> Self {
        Self { id: sub, name: preferred_username, email }
    }
}
