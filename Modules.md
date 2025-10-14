# 模組 (Modules)

```rs
// src/math.rs
#![allow(dead_code)]

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn div(a: i32, b: i32) -> i32 {
    a / b
}
```

```rs
// src/main.rs
mod math;

use math::{add, sub};

fn main() {
    let val = add(1, 6);
    println!("1 + 6 = {val}");
    // 1 + 6 = 7

    let val = sub(1, 6);
    println!("1 - 6 = {val}");
    // 1 - 6 = -5
}
```

## 模組樹

```rs
$ cargo install cargo-modules --locked
```

## 工作空間 (Workspaces)

### `pub(crate)`

### `pub(super)`

### `pub(in path)`
