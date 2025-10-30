# Rust Solutions

🦀 Rust Journey: From Zero to Hero, Open Source to Business Applications

---

## 程式設計 (Programming)

- [變數 (Variables)](./programming/Variables.md)
- [資料型別 (Data Types)](./programming/DataTypes.md)
- [函式 (Functions)](./programming/Functions.md)
- [控制流程 (Control Flow)](./programming/ControlFlow.md)
- [模組 (Modules)](./programming/Modules.md)
- [作用域 (Scopes)](./programming/Scopes.md)
- [屬性 (Attributes)](./programming/Attributes.md)
- [巨集 (Macros)](./programming/Macros.md)
- [智慧指標 (Smart Pointers)](./programming/SmartPointers.md)
- [非同步 (Asynchronous)](./programming/Asynchronous.md)
- [並行 (Concurrency)](./programming/Concurrency.md)
- [套件管理器 (Package Manager)](./programming/PackageManager.md)
- [錯誤處理 (Error Handling)](./programming/ErrorHandling.md)
- [數學 (Math)](./programming/Math.md)
- [日期和時間 (Date and Time)](./programming/DateAndTime.md)
- [正規表達式 (Regular Expressions)](./programming/RegularExpressions.md)
- [測試 (Testing)](./programming/Testing.md)

---

## 演算法 (Algorithms)

- [Beginning - Data Structures and Algorithms](./algorithms/Beginning.md)
  - [陣列 (Array)](./algorithms/Beginning.md#陣列-array)
  - [鏈結串列 (Linked List)](./algorithms/Beginning.md#鏈結串列-linked-list)
  - [堆疊 (Stack)](./algorithms/Beginning.md#堆疊-stack)
  - [佇列 (Queue)](./algorithms/Beginning.md#佇列-queue)
  - [雜湊表 (Hash Table)](./algorithms/Beginning.md#雜湊表-hash-table)
  - [二元樹 (Binary Tree)](./algorithms/Beginning.md#二元樹-binary-tree)
  - 二元搜尋樹 (Binary Search Tree)
  - [平衡樹 (Balanced Tree, AVL / Red-Black Tree)](./algorithms/Beginning.md#平衡樹-balanced-tree-avl--red-black-tree)
  - [堆積 (Heap)](./algorithms/Beginning.md#堆積-heap)
  - 字典樹 (Trie)
  - 圖 (Graph)
  - 排序 (Sorting)
  - 搜尋 (Searching)
  - 分治 (Divide and Conquer)
  - 回溯 (Backtracking)
  - 動態規劃 (Dynamic Programming)
  - 貪婪 (Greedy)
  - 位元操作 (Bit Manipulation)
- [Basic - LeetCode 75](./algorithms/Basic.md)
- [Advanced - Top Interview 150](./algorithms/Advanced.md)
- [Impactful - Top 100 Liked](./algorithms/Impactful.md)

---

## 命令列應用 (Command Line Applications)

Clap

- [應用程式 (Application)](./command-line/CommandLine.md#應用程式-application)
- [子命令 (Subcommands)](./command-line/CommandLine.md#子命令-subcommands)
- [表格 (Tables)](./command-line/CommandLine.md#表格-tables)
- [進展 (Progresses)](./command-line/CommandLine.md#進展-progresses)
- [顏色 (Colors)](./command-line/CommandLine.md#顏色-colors)
- [提示字元 (Prompts)](./command-line/CommandLine.md#提示字元-prompts)
- [非同步 (Asynchronous)](./command-line/CommandLine.md#非同步-asynchronous)
- [開箱 (Unboxing Crates)](./command-line/CommandLine.md#開箱-unboxing-crates)

---

## 網頁應用 (Web Applications)

Leptos

- [應用程式 (Application)](./web/Application.md)
- 元件 (Components)
  - 模板語法 (Template Syntax)
  - Reactivity
    - Signals / Computeds / Effects
  - Class and Style
    - Tailwind CSS
    - Scoped CSS
    - Icon
  - Props / Event handler props
  - Children
- Router
- State Management
- Data Fetching
  - Streams API
- Form Validation
- Web APIs + `leptos::document();` + `web-sys`
- 製作 UI 元件
- `leptos-use`
- 資料視覺化 (Data Visualization)
  - `charming` (ECharts)
- 多語言支援與國際化 (Internationalization)
- 伺服器端算繪 (Server-side Rendering)
  - `axum`
  - 後設資料 (Metadata)
- 雲端服務部署
  - 容器化 + Caddy Server + Docker
  - 部署到 Render
- 雲端平台部署
  - 容器化全代管部署
    - Google Cloud Run
    - Azure Container Apps
    - AWS Fargate
  - Kubernetes + Helm
    - Google Kubernetes Engine
    - Azure Kubernetes Service
    - Amazon Elastic Kubernetes Service
- 本地部署 (輕量化) + Kubernetes K3s
  - Raspberry Pi 5 Model B
- 跨平台原生應用 `tauri`
  - Pull to Refresh
  - Plugins
    - Store
    - Notifications
  - Android + 發布至 Google Play (Closed Testing)
  - Windows + 發布至 Microsoft Store (Package Flights)
  - iOS + 發布至 App Store (Apple TestFlight)
  - macOS + 發布至 App Store (Apple TestFlight)

---

## 伺服器應用 (Server Applications)

Axum

- [應用程式 (Application)](./server/Application.md)
- [路由 (Routing)](./server/Routing.md)
- [中介軟體 (Middleware)](./server/Middleware.md)
- [身分驗證 (Authentication)](./server/Authentication.md)
- [MongoDB 文件資料庫 (MongoDB Document Database)](./server/MongoDB.md)
- [MinIO S3 物件儲存 (MinIO S3 Object Storage)](./server/MinIO.md)
- [Redis 鍵值對資料庫 (Redis Key-Value Database)](./server/Redis.md)
- [資料流 (Streaming)](./server/Streaming.md)
- [WebSocket 雙向通訊 (WebSocket Full-duplex Interaction)](./server/WebSocket.md)
- 同構 (Isomorphic)
  - `leptos`
- 人工智慧代理 (AI Agents)
  - Gemini (DeepMind) / ChatGPT (OpenAI) / Claude (Anthropic) + Qdrant
    - `swiftide-*` + stream + `leptos`
    - Chat Memory
    - Tool Calling Agents
  - `candle-*` + Hugging Face + `qdrant-client` + Qdrant
- 提供服務使用 GraphQL (`async-graphql`)
- Email
  - MJML
- 多語言支援與國際化 (Internationalization)
- 雲端服務部署
  - 容器化 + Docker
  - 部署到 Render + MongoDB Atlas + IDrive e2
  - 部署到 Redis Cloud + Render's Background Workers
  - 部署到 Qdrant Managed Cloud
  - 申請設定 Gmail SMTP Server
- 雲端平台部署
  - 容器化全代管部署
    - Google Cloud Run
    - Azure Container Apps
    - AWS Fargate
  - 人工智慧代理遷移進平台
    - Gemini (DeepMind) on Google Cloud
    - ChatGPT (OpenAI) on Microsoft Azure
    - Claude (Anthropic) on AWS
  - Kubernetes + Helm
    - Google Kubernetes Engine
    - Azure Kubernetes Service
    - Amazon Elastic Kubernetes Service
- 本地部署 (輕量化) + Kubernetes K3s
  - Raspberry Pi 5 Model B

---

## 嵌入式應用 (Embedded Applications)

Raspberry Pi

- `gpio-cdev` + 麵包版 (Breadboard) + 電阻 (Resistor) + 發光二極體 (LED) + 按鈕 (Button)
- 8x8 LED 點矩陣 (8x8 LED Matrix)
- 光敏電阻 / 蜂鳴器 / 紅外線偵測 / 馬達
- 機械手臂 (Robot Arm Kit for Raspberry Pi)
- 相機 (Raspberry Pi Camera Module 3) + `libcamera`
- 觸控螢幕 (Raspberry Pi Display 2) + `tauri` + `leptos`
- AI 模型推理 + `candle-*`
- 微控制器 (Raspberry Pi Pico 2) + `embassy-rp`
- 感測器套件 (Yahboom Sensor Kit)
- UART 串列通訊
- MQTT 通訊協定 + `rumqttc`
- NFC Module 讀寫卡模組 PN532
