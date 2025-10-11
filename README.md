# Rust Solutions

🦀

---

## Rust Programming

- 變數 (Variables)
- 資料型別 (Data Types)
- 函式 (Functions)
- 控制流程 (Control Flow)
- ...
- 智慧指標 (Smart Pointers)

---

## ...

### 排序

```rs
fn main() {
    let mut vec = vec![1, 30, 4, 21, 100000];
    vec.sort();
    println!("{vec:?}");
    // [1, 4, 21, 30, 100000]
}
```

```rs
fn main() {
    let mut vec = vec![1, 30, 4, 21, 100000];
    vec.sort_by(|a, b| b.cmp(a));
    println!("{vec:?}");
    // [100000, 30, 21, 4, 1]
}
```

```rs
fn main() {
    let mut fruits = vec!["Apple", "pear", "Banana", "orange"];
    fruits.sort_by_key(|s| s.to_lowercase());
    println!("{fruits:?}");
    // ["Apple", "Banana", "orange", "pear"]
}
```

---

## 演算法 (Algorithms)

---

## 伺服器端應用

`axum`

---

## 嵌入式裝置應用

Raspberry Pi 5 + `gpio-cdev`
