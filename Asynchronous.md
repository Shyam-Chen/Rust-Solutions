# 非同步 (Asynchronous)

```rs
$ cargo add tokio -F full
```

```rs
async fn async_task() -> u32 {
    7
}

#[tokio::main]
async fn main() {
    let result = async_task().await;
    println!("{result}");
    // 7
}
```
