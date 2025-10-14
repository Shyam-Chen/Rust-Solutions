use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct HelloWorld {
    message: String,
}

async fn hello_world() -> Json<HelloWorld> {
    Json(HelloWorld {
        message: "Hello, World!".into(),
    })
}

#[tokio::main]
async fn main() {
    let api = Router::new().route("/hello-world", get(hello_world));

    let app = Router::new().nest_service("/api", api);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
