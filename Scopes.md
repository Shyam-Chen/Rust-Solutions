# 作用域 (Scopes)

## 所有權 (Ownership)

```rs
{
    let s = String::from("Hello, World!"); // 變數 s 被建立
    println!("{s}"); // s 在作用域內有效
} // s 離開作用域，其記憶體被自動釋放
```

### 移動所有權

```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // 移動所有權給 s2
    // println!("{s1}, World!"); // 當所有權被移動後，原變數 s1 就無效了，所以不能再使用它。
    println!("{s2}, World!");
}
```

### 借用 (引用)

```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1; // 借用 s1 的所有權，而不是移動
    println!("{s1}, World!"); // s1 仍然有效
    println!("{s2}, World!"); // 使用借用的 s2
}
```

### 複製 (克隆, Clone)

```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // 複製所有權
    println!("{s1}, World!");
    println!("{s2}, World!");
}
```

### 移動 (Move)

```rs
fn main() {
    let s = String::from("Hello");
    let closure = || println!("{s}"); // 閉包只讀取外部變數 s
    closure(); // 使用閉包
    println!("{s}"); // s 仍然有效
}
```

移轉:

```rs
fn main() {
    let s = String::from("Hello");
    let closure = move || println!("{s}"); // 閉包只讀取外部變數 s
    closure(); // 使用閉包
    // println!("{s}"); // 已被移轉，所以無效
}
```

#### 複製 (拷貝, Copy) 型別的特徵

```rs
fn main() {
    let val = 7;
    let closure = || println!("{val}"); // 閉包只讀取外部變數 val
    closure(); // 使用閉包
    println!("{val}"); // val 仍然有效
}
```

移轉為直接複製值:

```rs
fn main() {
    let val = 7;
    let closure = move || println!("{val}");
    closure();
    println!("{val}"); // val 仍然有效
}
```

## 資源獲取即初始化 (RAII)

```rs
{
    let file = std::fs::File::open("data.txt").unwrap();
    // 使用 file ...
} // file 離開作用域，自動關閉文件
```
