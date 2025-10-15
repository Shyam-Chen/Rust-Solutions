# 並行 (Concurrency)

```rs
use std::thread;

fn main() {
    println!("主執行緒 - 1");

    let handle = thread::spawn(|| {
        println!("來自另一個執行緒的問候！");
    });

    println!("主執行緒 - 2");

    handle.join().unwrap();

    println!("主執行緒 - 3");
}
```
