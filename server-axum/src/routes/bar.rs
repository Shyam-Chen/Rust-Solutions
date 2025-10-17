use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HelloWorld {
    message: String,
}

async fn hello_world() -> Json<HelloWorld> {
    Json(HelloWorld {
        message: "Hello, World!".into(),
    })
}

pub fn router() -> Router {
    Router::new().route("/hello-world", get(hello_world))
}
