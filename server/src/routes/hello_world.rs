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

pub fn router() -> Router {
    Router::new()
        .route(
            "/hello-world",
            get(|| async {
                Json(HelloWorld {
                    message: "Hello, World!".into(),
                })
            }),
        )
        .route(
            "/hello-world",
            post(|Json(payload): Json<HelloPayload>| async move {
                let name = payload.name;
                let message = format!("Hello, {name}!");
                Json(HelloWorld { message })
            }),
        )
}
