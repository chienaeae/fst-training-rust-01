use saffron_repository::model::GenericLogic;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Logic {
    #[serde(flatten)]
    pub metadata: GenericLogic,

    pub linked_card: Vec<LinkedLogicInfo>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedLogicInfo {
    pub id: Uuid,
    pub card_id: Uuid,
}
