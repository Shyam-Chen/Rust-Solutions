# 非同步 (Asynchronous)

```sh
$ cargo add futures
```

```rs
use futures::executor::block_on;

async fn add_one(val: u32) -> u32 {
    val + 1
}

async fn square_multiply(val: u32) -> u32 {
    val * val
}

async fn async_main() {
    let val = add_one(10).await;
    let result = square_multiply(val).await;
    println!("{result}");
    // 121
}

fn main() {
    block_on(async_main());
}
```

## 非同步執行階段 (Asynchronous Runtime)

```sh
$ cargo add tokio -F full
```

```rs
async fn add_one(val: u32) -> u32 {
    val + 1
}

async fn square_multiply(val: u32) -> u32 {
    val * val
}

#[tokio::main]
async fn main() {
    let val = add_one(10).await;
    let result = square_multiply(val).await;
    println!("{result}");
    // 121
}
```
