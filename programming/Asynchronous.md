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

等到全部履行 (或一個拒絕):

```rs
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let task1 = async {
        println!("Task 1 開始");
        time::sleep(Duration::from_secs(1)).await; // 模擬 1 秒的延遲
        println!("Task 1 完成");
        Ok::<_, &str>(1) // 成功回傳結果
    };

    let task2 = async {
        println!("Task 2 開始");
        time::sleep(Duration::from_secs(3)).await; // 模擬 3 秒的延遲
        println!("Task 2 完成");
        Ok::<_, &str>(2) // 成功回傳結果
    };

    let task3 = async {
        println!("Task 3 開始");
        time::sleep(Duration::from_secs(2)).await; // 模擬 2 秒的延遲
        println!("Task 3 完成");
        Ok::<_, &str>(3) // 成功回傳結果
    };

    let results = tokio::try_join!(task1, task2, task3);
    println!("results = {results:?}");
}
// Task 1 開始
// Task 2 開始
// Task 3 開始
// Task 1 完成
// Task 3 完成
// Task 2 完成
// results = Ok((1, 2, 3))
```

```rs
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let task1 = async {
        println!("Task 1 開始");
        time::sleep(Duration::from_secs(1)).await; // 模擬 1 秒的延遲
        println!("Task 1 完成");
        Ok::<_, &str>(1) // 成功回傳結果
    };

    let task2 = async {
        println!("Task 2 開始");
        time::sleep(Duration::from_secs(3)).await; // 模擬 3 秒的延遲
        println!("Task 2 完成");
        Ok::<_, &str>(2) // 成功回傳結果
    };

    let task3 = async {
        println!("Task 3 開始");
        time::sleep(Duration::from_secs(2)).await; // 模擬 2 秒的延遲
        println!("Task 3 出錯");
        Err::<i32, _>("Error!") // 模擬失敗狀況
    };

    let results = tokio::try_join!(task1, task2, task3);
    println!("results = {results:?}");
}
// Task 1 開始
// Task 2 開始
// Task 3 開始
// Task 1 完成
// Task 3 出錯
// results = Err("Error!")
```

錯誤處裡:

```rs
use anyhow::{Result, anyhow};
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let task1 = async {
        println!("Task 1 開始");
        time::sleep(Duration::from_secs(1)).await;
        println!("Task 1 完成");
        Ok::<i32, anyhow::Error>(1) // 改用 anyhow 錯誤類型
    };

    let task2 = async {
        println!("Task 2 開始");
        time::sleep(Duration::from_secs(3)).await;
        println!("Task 2 完成");
        Ok::<i32, anyhow::Error>(1) // 改用 anyhow 錯誤類型
    };

    let task3 = async {
        println!("Task 3 開始");
        time::sleep(Duration::from_secs(2)).await;
        println!("Task 3 出錯");
        Err::<i32, anyhow::Error>(anyhow!("Error!")) // 回傳帶錯誤訊息的 anyhow::Error
    };

    let results = tokio::try_join!(task1, task2, task3);

    match results {
        // 所有任務成功執行
        Ok((result1, result2, result3)) => {
            println!("task1 = {result1}, task2 = {result2}, task3 = {result3}");
        }
        // 任務中有錯誤
        Err(error) => {
            println!("{error}");
        }
    }

    Ok(())
}
```
