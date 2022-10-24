mod card;
mod user;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::{
    card::{Card, CreateCardRequest, UpdateCardRequest},
    user::User,
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct DeleteInfo {
    pub id: Uuid,
}

impl DeleteInfo {
    #[must_use]
    pub const fn new(id: Uuid) -> Self { Self { id } }
}
