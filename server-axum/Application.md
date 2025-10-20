# 應用程式 (Application)

```sh
$ cargo new my-axum-project
$ cd my-axum-project
```

```sh
$ cargo add axum
$ cargo add tokio -F full
```

```rs
// src/main.rs
use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn hello_world() -> String {
    "Hello, World!".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello-world", get(hello_world));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
```

```sh
$ curl --request GET --url http://localhost:3000/hello-world
Hello, World!
```

---

全域安裝一次:

```sh
$ cargo install cargo-make --locked
```

專案下執行:

```sh
$ cargo make dev
```
