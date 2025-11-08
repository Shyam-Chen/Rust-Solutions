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
    let mut router = Router::new();

    router = router.route(
        "/",
        get(|| async {
            Json(HelloWorld {
                message: "Hello, World!".into(),
            })
        }),
    );

    router = router.route(
        "/",
        post(|Json(payload): Json<HelloPayload>| async move {
            let message = format!("Hello, {}!", payload.name);
            Json(HelloWorld { message })
        }),
    );

    router
}

pub fn router() -> Router {
    Router::new().nest("/hello-world", handler())
}
