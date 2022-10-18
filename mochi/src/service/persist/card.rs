// use crate::{
//     domain::model,
//     service::persist::error::Result,
// };
// use async_trait::async_trait;
// use uuid::Uuid;

// #[async_trait]
// pub trait CardPersistService: Send + Sync {
//     async fn create(&self, card: &model::CreateCardRequest) -> Result<Uuid>;

//     async fn get_all(&self) -> Result<Vec<model::Card>>;

//     async fn get_by_id(&self) -> Result<Option<model::Card>>;

//     async fn update_by_id(&self) -> Result<Option<model::Card>>;

//     async fn delete_by_id(&self) -> Result<Option<Uuid>>;
// }

// #[async_trait]
// impl CardPersistService for DefaultPersistService {
//   async fn create(&self, model::CreateCardRequest {name, description}:
// &model::CreateCardRequest) -> Result<Uuid>{     sqlx::query_file!("sql/card/
// insert.sql", name, description)   }
