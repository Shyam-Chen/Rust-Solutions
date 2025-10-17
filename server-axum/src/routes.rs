use axum::Router;

mod bar;
mod foo;
mod hello_world;

pub fn router() -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .merge(hello_world::router())
            .merge(foo::router())
            .merge(bar::router()),
    )
}
