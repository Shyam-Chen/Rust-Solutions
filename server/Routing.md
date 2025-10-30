# 路由 (Routing)

```rs
use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct HelloWorld {
    message: String,
}

fn handler() -> Router {
    Router::new().route(
        "/",
        get(|| async {
            Json(HelloWorld {
                message: "Hello, World!".into(),
            })
        }),
    )
}

pub fn router() -> Router {
    Router::new().nest("/hello-world", handler())
}
```

## 查詢字串 (Query Strings)

## 參數 (Parameters)

## 方法 (Methods)

## 負載 (Payloads)
