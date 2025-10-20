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
