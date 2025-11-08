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

## 開發模式

全域安裝一次:

```sh
$ cargo install cargo-make --locked
```

```sh
$ cargo install watchexec-cli --locked
```

建立 `Makefile.toml`:

```toml
# Makefile.toml
[tasks.dev]
command = "watchexec"
args = ["-r", "-w=src", "cargo", "run"]
```

專案下執行:

```sh
$ cargo make dev
```

將 World 改成 Rust 查看變化:

```rs
// src/main.rs

async fn hello_world() -> String {
    "Hello, Rust!".to_string()
}
```

```sh
$ curl --request GET --url http://localhost:3000/hello-world
Hello, Rust!
```

---

序列化和反序列化:

```sh
$ cargo add serde -F derive
```

```rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct HelloWorld {
    message: String,
}
```
