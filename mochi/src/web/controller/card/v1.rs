use axum::{extract::Path, http::StatusCode, response::Json};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::model::{Card, DeleteInfo},
    response::EncapsulatedJson,
    web::error,
};

pub async fn create(Json(card): Json<Card>) -> error::Result<EncapsulatedJson<Card>> {
    tokio::spawn(async {}).await.unwrap();
    println!("Create a card");
    Ok(EncapsulatedJson::ok(card).status_code(StatusCode::CREATED))
}

pub async fn get_all() -> error::Result<EncapsulatedJson<Vec<Card>>> {
    tokio::spawn(async {}).await.unwrap();
    println!("Get cards");
    let data = vec![
        Card {
            id: Uuid::new_v4(),
            name: "Test Name".to_string(),
            description: "Test Description".to_string(),
            creation_timestamp: Utc::now(),
        },
        Card {
            id: Uuid::new_v4(),
            name: "Test Name".to_string(),
            description: "Test Description".to_string(),
            creation_timestamp: Utc::now(),
        },
        Card {
            id: Uuid::new_v4(),
            name: "Test Name".to_string(),
            description: "Test Description".to_string(),
            creation_timestamp: Utc::now(),
        },
    ];

    Ok(EncapsulatedJson::ok(data))
}

pub async fn get_by_id(Path(id): Path<Uuid>) -> error::Result<EncapsulatedJson<Card>> {
    tokio::spawn(async {}).await.unwrap();
    println!("Get a card of {}", id);
    let data = Card {
        id: Uuid::new_v4(),
        name: "Test Name".to_string(),
        description: "Test Description".to_string(),
        creation_timestamp: Utc::now(),
    };

    Ok(EncapsulatedJson::ok(data).status_code(StatusCode::CREATED))
}

pub async fn update_by_id(Path(id): Path<Uuid>) -> error::Result<EncapsulatedJson<Card>> {
    tokio::spawn(async {}).await.unwrap();
    println!("Update a card of {}", id);
    let data = Card {
        id: Uuid::new_v4(),
        name: "Test Name".to_string(),
        description: "Test Description".to_string(),
        creation_timestamp: Utc::now(),
    };

    Ok(EncapsulatedJson::ok(data).status_code(StatusCode::CREATED))
}

pub async fn delete_by_id(Path(id): Path<Uuid>) -> error::Result<EncapsulatedJson<DeleteInfo>> {
    tokio::spawn(async {}).await.unwrap();
    println!("Delete a card of {}", id);
    let data = DeleteInfo { id: Uuid::new_v4() };

    Ok(EncapsulatedJson::ok(data).status_code(StatusCode::CREATED))
}
