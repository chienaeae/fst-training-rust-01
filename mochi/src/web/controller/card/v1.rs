use axum::http::StatusCode;

use crate::{response::EncapsulatedJson, web::error};

pub async fn create() -> error::Result<EncapsulatedJson<String>> {
    let handle = tokio::spawn(async {
        // Do some async work
        "Create a card"
    });

    // Do some other work
    let out = handle.await.unwrap();
    println!("{}", out);

    Ok(EncapsulatedJson::ok("Create a card".to_string()).status_code(StatusCode::CREATED))
}
