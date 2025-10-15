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

## 字串 (Strings)

### 靜態字串 (String Slices)

### 動態字串 (Strings)

```rs
fn main() {
    let text = String::from("Hello, World!");
    println!("{text}");
    // Hello, World!

    let mut text = String::new();
    text.push_str("Hello, World!");
    println!("{text}");
    // Hello, World!

    let text = "Hello, World!".to_string();
    println!("{text}");
    // Hello, World!

    let text: String = "Hello, World!".into(); // 必須明確指定型別
    println!("{text}");
    // Hello, World!

    let world = "World";
    let hello_world = format!("Hello, {world}!");
    println!("{hello_world}");
    // Hello, World!
}
```

## 陣列 (Arrays)

### 靜態陣列 (Arrays)

```rs
fn main() {
    let arr = [1, 2, 3, 4, 5]; // 型別推斷為 [i32; 5]
    println!("{arr:?}");
    // [1, 2, 3, 4, 5]

    let zeros = [0; 5];
    println!("{zeros:?}");
    // [0, 0, 0, 0, 0]
}
```

轉成切片:

```rs
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("{slice:?}");
    // [2, 3, 4]
}
```

轉成動態陣列:

```rs
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut vec = arr.to_vec();
    vec.push(6);
    println!("{vec:?}");
    // [1, 2, 3, 4, 5, 6]
}
```

### 動態陣列 (Vectors)

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

#### 疊代器 (Iterators)

```rs
fn main() {
    let vec = vec![1, 2, 3];

    for num in vec {
        println!("{num}");
    }

    // println!("{vec:?}"); // vec 的所有權在 for 迴圈中被消耗掉 (即已移動)
}
```

引用:

```rs
fn main() {
    let vec = vec![1, 2, 3];

    for num in &vec { // 引用
        println!("{num}");
    }

    println!("{vec:?}");
}
```

不可變:

```rs
fn main() {
    let vec = vec![1, 2, 3];
    vec.iter().for_each(|num| println!("{num}"));
    println!("{vec:?}");
}
```

可變:

```rs
fn main() {
    let mut vec = vec![1, 2, 3];

    vec.iter_mut().for_each(|num| {
        *num += 1;
        println!("{num}");
    });

    println!("{vec:?}");
}
```

同 `for` 迴圈:

```rs
fn main() {
    let vec = vec![1, 2, 3];
    vec.into_iter().for_each(|num| println!("{num}"));
    // println!("{vec:?}");
}
```

## 元組 (Tuples)

## 結構 (Structures)

## 列舉 (Enumerations)

### 內建列舉

#### `Option`

https://doc.rust-lang.org/std/option/enum.Option.html

```rs
let some_value: Option<i32> = Some(7); // 表示存在值 7
let none_value: Option<i32> = None; // 表示值不存在
```

#### `Result`

https://doc.rust-lang.org/std/result/enum.Result.html

#### `Poll`

https://doc.rust-lang.org/std/task/enum.Poll.html

### 自訂列舉

```rs
#![allow(dead_code)]

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let dir = Direction::Up;

    match dir {
        Direction::Up => println!("Up"),
        Direction::Right => println!("Right"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
    }
}
// Up
```

## 雜湊映射 (Hash Maps)

## 雜湊集合 (Hash Sets)

```rs
use std::collections::HashSet;

fn main() {
    let mut set: HashSet<&str> = HashSet::new();

    set.insert("apple");
    set.insert("banana");
    set.insert("orange");

    // 重複的元素不會被加入
    set.insert("apple");

    println!("{set:?}");
    // {"apple", "banana", "orange"}
}
```
