use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct HelloWorld {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct HelloPayload {
    name: String,
}

fn handler() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async {
                Json(HelloWorld {
                    message: "Hello, World!".into(),
                })
            }),
        )
        .route(
            "/",
            post(|Json(payload): Json<HelloPayload>| async move {
                let name = payload.name;
                let message = format!("Hello, {name}!");
                Json(HelloWorld { message })
            }),
        )
}

pub fn router() -> Router {
    Router::new().nest("/hello-world", handler())
}
