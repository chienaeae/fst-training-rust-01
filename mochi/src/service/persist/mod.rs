use sqlx::{Pool, Postgres};

mod card;
pub mod error;

#[derive(Clone)]
pub struct DefaultPersistService {
    postgres: Pool<Postgres>,
}

impl DefaultPersistService {
    pub const fn new(postgres: Pool<Postgres>) -> Self { Self { postgres } }
}
