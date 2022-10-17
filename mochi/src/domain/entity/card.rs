use chrono::{NaiveDateTime, TimeZone, Utc};
use uuid::Uuid;

use crate::domain::model;

#[derive(Clone, Debug)]
pub struct Card {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub creation_timestamp: NaiveDateTime,
}

impl From<Card> for model::Card {
    #[inline]
    fn from(Card { id, name, description, creation_timestamp }: Card) -> Self {
        Self {
            id,
            name,
            description,
            creation_timestamp: Utc.from_utc_datetime(&creation_timestamp),
        }
    }
}
