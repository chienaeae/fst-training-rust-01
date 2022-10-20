use sqlx::Postgres;

use crate::service::{CardPersistService, DefaultPersistService};

pub trait Context: Clone + Send + Sync {
    type CardPersistService: CardPersistService;

    fn card_persist_service(&self) -> &Self::CardPersistService;
}

#[derive(Clone)]
pub struct DefaultContext {
    persist_service: DefaultPersistService,
}

impl DefaultContext {
    #[must_use]
    pub const fn new(postgres: sqlx::Pool<Postgres>) -> Self {
        Self { persist_service: DefaultPersistService::new(postgres) }
    }
}

impl Context for DefaultContext {
    type CardPersistService = DefaultPersistService;

    fn card_persist_service(&self) -> &Self::CardPersistService { &self.persist_service }
}
