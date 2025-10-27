# 套件管理器 (Package Manager)

- Cargo
- Crates [crates.io](https://crates.io/)

## 套件架構 (Package Structure)

- 執行檔入口: `src/main.rs`
- 函式庫入口: `src/lib.rs`
- 二進制執行檔: `src/bin/*.rs`
- 範例檔: `examples/*.rs`
- 整合測試: `tests/*.rs`
- 基準測試: `benches/*.rs`

```sh
$ cargo run
```

```sh
$ cargo run --bin [<NAME>]
```

```sh
$ cargo run --example [<NAME>]
```

```sh
$ cargo test
```

```sh
$ cargo bench
```

## 工作空間 (Workspaces)

```sh
$ cargo new my-workspace
$ cd my-workspace
```

```toml
# Cargo.toml
[workspace]
members = [
  "crate-foo", # 第一個子專案
  "crate-bar", # 第二個子專案
]
```

子專案:

```sh
$ cargo new crate-foo
```

```sh
$ cargo new crate-bar
```

建置整個工作空間:

```sh
$ cargo build
```

只建置指定的套件:

```sh
$ cargo build -p crate-foo
```

### 關聯性

```toml
# crate-foo/Cargo.toml
[package]
name = "crate-foo"
version = "0.1.0"
edition = "2024"

[dependencies]
crate-bar = { path = "../crate-bar" }
```

#### 同屬

```toml
# Cargo.toml
[workspace]
members = [
  "crate-foo", # 第一個子專案
  "crate-bar", # 第二個子專案
]

[workspace.dependencies]
crate-bar = { path = "./crate-bar" }
```

```toml
# crate-foo/Cargo.toml
[package]
name = "crate-foo"
version = "0.1.0"
edition = "2024"

[dependencies]
crate-bar = { workspace = true }
```
