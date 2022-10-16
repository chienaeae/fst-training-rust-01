use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::model::{Card, DeleteInfo},
    response::EncapsulatedJson,
    web::error,
};

pub async fn create() -> error::Result<EncapsulatedJson<Card>> {
    let handle = tokio::spawn(async {
        // Do some async work
        "Create a card"
    });

    // Do some other work
    let out = handle.await.unwrap();
    println!("{}", out);

    let data = Card {
        id: Uuid::new_v4(),
        name: "Test Name".to_string(),
        description: "Test Description".to_string(),
        creation_timestamp: Utc::now(),
    };

    Ok(EncapsulatedJson::ok(data).status_code(StatusCode::CREATED))
}

pub async fn get_all() -> error::Result<EncapsulatedJson<Vec<Card>>> {
    let handle = tokio::spawn(async {
        // Do some async work
        "Get cards"
    });

    // Do some other work
    let out = handle.await.unwrap();
    println!("{}", out);

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

pub async fn get_by_id() -> error::Result<EncapsulatedJson<Card>> {
    let handle = tokio::spawn(async {
        // Do some async work
        "Get a card"
    });

    // Do some other work
    let out = handle.await.unwrap();
    println!("{}", out);

    let data = Card {
        id: Uuid::new_v4(),
        name: "Test Name".to_string(),
        description: "Test Description".to_string(),
        creation_timestamp: Utc::now(),
    };

    Ok(EncapsulatedJson::ok(data).status_code(StatusCode::CREATED))
}

pub async fn update_by_id() -> error::Result<EncapsulatedJson<Card>> {
    let handle = tokio::spawn(async {
        // Do some async work
        "Update a card"
    });

    // Do some other work
    let out = handle.await.unwrap();
    println!("{}", out);

    let data = Card {
        id: Uuid::new_v4(),
        name: "Test Name".to_string(),
        description: "Test Description".to_string(),
        creation_timestamp: Utc::now(),
    };

    Ok(EncapsulatedJson::ok(data).status_code(StatusCode::CREATED))
}

pub async fn delete_by_id() -> error::Result<EncapsulatedJson<DeleteInfo>> {
    let handle = tokio::spawn(async {
        // Do some async work
        "Delete a card"
    });

    // Do some other work
    let out = handle.await.unwrap();
    println!("{}", out);

    let data = DeleteInfo { id: Uuid::new_v4() };

    Ok(EncapsulatedJson::ok(data).status_code(StatusCode::CREATED))
}
