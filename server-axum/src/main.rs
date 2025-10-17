use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    let app = routes::router();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
