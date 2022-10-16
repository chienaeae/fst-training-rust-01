use axum::{routing, Router};

pub fn v1() -> Router {
    Router::new().nest(
        "/v1/card",
        Router::new()
            .route(
                "/",
                routing::post(|| async {
                    println!("Create a card");
                })
                .get(|| async {
                    println!("Get all cards");
                }),
            )
            .route(
                "/:id",
                routing::get(|| async {
                    println!("Get a card by id");
                })
                .put(|| async {
                    println!("Update a card by id");
                })
                .delete(|| async {
                    println!("Delete a card by id");
                }),
            ),
    )
}
