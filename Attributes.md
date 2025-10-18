# 屬性 (Attributes)

## 內建屬性

### `#[derive]`

告訴編譯器實作指定的特徵 (Trait)。

```rs
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = p1.clone(); // derive(Clone)
    println!("{p1:?}"); // derive(Debug)
    // Point { x: 3, y: 4 }
    println!("相等嗎？{}", p1 == p2); // derive(PartialEq)
    // 相等嗎？true
}
```

```rs
#![allow(dead_code)]

#[derive(Debug, Default)]
struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
    fullscreen: bool,
}

fn main() {
    let wc = WindowConfig::default();
    println!("{wc:?}");
    // WindowConfig { title: "", width: 0, height: 0, fullscreen: false }
}
```

用在 `enum`:

```rs
#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let dir = Direction::Up;
    println!("{dir:?}", );
    // Up

    if dir == Direction::Up {
        println!("Up");
    }
}
```

| Trait                | 說明                            | 範例用途                  |
| -------------------- | ------------------------------- | ------------------------- |
| `Debug`              | 允許使用 `{:?}` 印出內容        | `println!("{obj:?}");`    |
| `Clone`              | 完整複製 (深克隆)               | `let b = a.clone();`      |
| `Copy`               | 位元複製 (淺拷貝)               | `let b = a; // a 仍可用`  |
| `PartialEq`          | 允許 `==`、`!=` 比較            | `if a == b { ... }`       |
| `Eq`                 | 更嚴格的相等比較 (需滿足反身性) | 與 `PartialEq` 一起使用   |
| `PartialOrd`         | 允許 `<`、`>` 比較              | 排序時                    |
| `Ord`                | 全排序 (結合 Eq + PartialOrd)   | `vec.sort()`              |
| `Hash`               | 可用於雜湊結構                  | `HashMap<Key, Value>`     |
| `Default`            | 提供預設值                      | `T::default()`            |
| `serde::Serialize`   | JSON 序列化                     | 與 `serde` crate 一起使用 |
| `serde::Deserialize` | JSON 反序列化                   | 與 `serde` crate 一起使用 |

#### 自訂 Derive Macros `#[derive(CustomDerive)]`

```sh
$ cargo new hello-derive --lib
```

```toml
# Cargo.toml
[package]
name = "hello-derive"
version = "0.1.0"
edition = "2024"

[lib]
proc-macro = true
```

```rs
// src/lib.rs
use proc_macro::TokenStream;
```

```sh
$ cargo add syn
```

### `#[allow]` 和 `#[deny]`

全域: `#![allow(dead_code)]`

```rs
#![allow(dead_code)]

fn unused_function() {
    println!("This function is never used");
}

struct UnusedStruct {
    value: i32,
}

fn main() {
    println!("No warnings will be displayed.");
}
```

局部: `#[allow(dead_code)]`

```rs
#[allow(dead_code)]
fn unused_function() {
    println!("This function is never used");
}

#[allow(dead_code)]
struct UnusedStruct {
    value: i32,
}

fn main() {
    println!("No warnings will be displayed.");
}
```

### `#[inline]`

### `#[cfg]` 和 `#[cfg_attr]`

### `#[non_exhaustive]`

## 自訂屬性 (Custom Attributes (程序式巨集, Procedural Macros))
