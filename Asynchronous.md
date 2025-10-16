# 非同步 (Asynchronous)

```sh
$ cargo add futures
```

## (非同步執行階段) Asynchronous Runtime

```sh
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
