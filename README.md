# Rust Solutions

🦀 Rust 101 -> Open Source -> Business

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
- [套件管理器 (Package Manager)](./PackageManager.md)
- [數學 (Math)](./Math.md)
- [日期和時間 (Date and Time)](./DateAndTime.md)
- [正規表達式 (Regular Expressions)](./RegularExpressions.md)
- [測試 (Testing)](./Testing.md)

---

## 演算法 (Algorithms)

- [Basic - LeetCode 75](./algorithms-leetcode/Basic.md)
- Advanced - Top Interview 150
- [Impactful - Top 100 Liked](./algorithms-leetcode/Impactful.md)

---

## 命令列應用 (Command Line Applications)

Clap

- [應用程式 (Application)](./command-line/Application.md)
- 表格 (Tables)
- 進度條 (Progress Bars)
- 顏色 (Colors)
- 提示字元 (Prompts)
- Excellent CLI tools
  - `bat`
  - `oha`

---

## 網頁應用 (Web Applications)

Leptos

- [應用程式 (Application)](./web-leptos/Application.md)
- Components
  - Props / Event handler props
  - Children
- Reactivity
  - Signals / Computeds / Effects
- CSS
  - Tailwind CSS
  - Scoped
- Router
- `leptos::document();` / `web-sys`
  - DOM
- `leptos-use`
- `charming` (ECharts)
- 雲端服務部署
  - 容器化 + Caddy Server + Docker
  - 部署到 Render
- 雲端平台部署
  - 容器化全代管部署 + Google Cloud Run / Azure Container Apps
  - Google Kubernetes Engine / Azure Kubernetes Service
- 本地部署 (輕量化) + Kubernetes K3s
  - Raspberry Pi 5 Model B
- `tauri`
  - Plugins: Notifications
  - Android + 發布至 Google Play (Closed Testing)
  - Windows + 發布至 Microsoft Store (Package Flights)
  - iOS + 發布至 App Store (Apple TestFlight)
  - macOS + 發布至 App Store (Apple TestFlight)

---

## 伺服器端網頁應用 (Server-side Web Applications)

Axum

- [應用程式 (Application)](./server-axum/Application.md)
- Router
- Middleware
- JWT (`jsonwebtoken`) + auth
- `mongodb` + crud operations
- `minio` + file uploads
- `redis` + cache + MQ + worker
- `candle-*` + Hugging Face + `qdrant-client` + Qdrant
- 伺服器端算繪 (Server-side Rendering)
  - `leptos`
- 雲端服務部署
  - 容器化 + Docker
  - 部署到 Render + MongoDB Atlas + IDrive e2
  - 部署到 Redis Cloud + Render's Background Workers
  - 部署到 Qdrant Managed Cloud
- 雲端平台部署
  - 容器化全代管部署 + Google Cloud Run / Azure Container Apps
  - Google Kubernetes Engine / Azure Kubernetes Service + Helm
- 本地部署 (輕量化) + Kubernetes K3s
  - Raspberry Pi 5 Model B + NVIDIA RTX A400

---

## 嵌入式裝置應用 (Embedded Device Applications)

Raspberry Pi

- `gpio-cdev` + 麵包版 (Breadboard) + 電阻 (Resistor) + 發光二極體 (LED) + 按鈕 (Button)
- 8x8 LED 點矩陣 (8x8 LED Matrix)
- 光敏電阻 / 蜂鳴器 / 紅外線偵測 / 馬達
- 機械手臂 (Robot Arm Kit for Raspberry Pi)
- 相機 (Raspberry Pi Camera Module 3) + `libcamera`
- 觸控螢幕 (Raspberry Pi Display 2) + `tauri` + `leptos`
- `candle-*` + Raspberry Pi 5 Model B + NVIDIA RTX A400
- Raspberry Pi Pico 2 + `embassy-rp`
