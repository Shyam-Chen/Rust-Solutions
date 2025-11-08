use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = routes::router()
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
