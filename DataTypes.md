# 資料型別 (Data Types)

## 數字 (Numbers)

- 整數型別 (Integer Types):
  - 有正負號 (Signed): `i32`
  - 只有正號 (Unsigned): `u32`
- 浮點數型別 (Floating-Point Types): `f64`

```rs
fn main() {
    // 自動推斷 (Type Inference)
    let x = 7; // i32
    let y = 7.0; // f64
    println!("{x}, {y}");
    // 7, 7

    // 明確指定 (Explicit Type Annotation)
    let x: i32 = 7;
    let y: f64 = 7.0;
    println!("{x}, {y}");
    // 7, 7

    // 型別後綴 (Type Suffix)
    let x = 7_i32;
    let y = 7_f64;
    println!("{x}, {y}");
    // 7, 7

    // 型別強轉
    let x = 7_i32;
    let y = 7_f64;
    let z = x as f64 + y;
    println!("{z}");
    // 14
}
```

```rs
fn main() {
    let val = 3.14159;

    let n = (val * 100_f64).round() / 100_f64;
    println!("{n}");
    // 3.14

    let s = format!("{val:.2}");
    println!("{s}");
    // 3.14
}
```

## 布林 (Booleans)

```rs
fn main() {
    let disabled = false;
    let has_permission: bool = true;
    println!("{disabled}, {has_permission}");
    // false, true
}
```

## 字元 (Characters)

```rs
fn main() {
    let c = 'c';
    println!("{c}");
    // c

    let c: char = 'c';
    println!("{c}");
    // c

    let c = '🦀';
    println!("{c}");
    // 🦀
}
```

## 字串

### 靜態字串

### 動態字串

## 陣列

### 靜態陣列

### 動態陣列

#### 排序

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

## 元組

## 結構 (Structures)

struct

## 列舉 (Enums)

enum

## union

## static
