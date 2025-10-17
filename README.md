# Rust Solutions

🦀 Rust 101 -> 777

---

## 程式設計 (Programming)

- [變數 (Variables)](./Variables.md)
- [資料型別 (Data Types)](./DataTypes.md)
- [函式 (Functions)](./Functions.md)
- [控制流程 (Control Flow)](./ControlFlow.md)
- [模組 (Modules)](./Modules.md)
- [作用域 (Scopes)](./Scopes.md)
- [屬性 (Attributes)](./Attributes.md)
- [巨集 (Macros)](./Macros.md)
- [智慧指標 (Smart Pointers)](./SmartPointers.md)
- [非同步 (Asynchronous)](./Asynchronous.md)
- [並行 (Concurrency)](./Concurrency.md)
- [日期和時間 (Date and Time)](./DateAndTime.md)
- [正規表達式 (Regular Expressions)](./RegularExpressions.md)
- [測試 (Testing)](./Testing.md)

---

## 演算法 (Algorithms)

- [Basic - LeetCode 75](./algorithms-leetcode/Basic.md)
- Advanced - Top Interview 150
- Impactful - Top 100 Liked

---

## 伺服器端網頁應用 (Server-side Web Applications)

全域安裝一次:

```sh
$ cargo install cargo-make --locked
```

專案下執行:

```sh
$ cargo make dev
```

`axum`

- hello-world
- JWT (`jsonwebtoken`) + auth
- `mongodb` + crud operations
- `minio` + file uploads
- `redis` + cache + MQ + worker
- `candle-*` + Hugging Face + `qdrant-client` + Qdrant
- ...
- 容器化 + Docker
- 部署到 Render + MongoDB Atlas + IDrive e2
- 部署到 Redis Cloud + Render's Background Workers ($)
- 部署到 Qdrant Managed Cloud

---

## 嵌入式裝置應用 (Embedded Device Applications)

Raspberry Pi 5 ($) + `gpio-cdev` + $$

- 麵包版 (Breadboard) + 電阻 (Resistor) + 發光二極體 (LED) + 按鈕 (Button)
- 8x8 LED 點矩陣 (8x8 LED Matrix)
- ...
- 機械手臂 (Robot Arm Kit for Raspberry Pi)
- 相機 (Raspberry Pi Camera Module 3) + `libcamera`
- 觸控螢幕 (Raspberry Pi Display 2) + `tauri`
