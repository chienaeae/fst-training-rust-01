use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,

    pub name: String,

    pub email: String,
}

impl User {
    #[must_use]
    pub fn new(id: Uuid, name: &str, email: &str) -> Self {
        Self { id, name: name.to_string(), email: email.to_string() }
    }
}
