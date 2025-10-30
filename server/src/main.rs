use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod routes;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = routes::router().layer(cors);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
