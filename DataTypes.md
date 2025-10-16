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

小數點後兩位:

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

可讀性:

```rs
fn main() {
    let val = 1_000_000;
    println!("{val}");
    // 1000000
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

### 靜態字串 (String Slices) `&str`

```rs
fn main() {
    let s = "Hello, World!";
    println!("{s}");
    // Hello, World!

    let first_word = &s[0..5];
    println!("{first_word}");
    // Hello
}
```

當函式的參數只是給內部函式讀取用時:

```rs
fn greet(name: &str) {
    println!("Hello, {name}!",);
}

fn main() {
    let my_name = String::from("Alice");
    greet(&my_name); // Hello, Alice!
    greet("Bob"); // Hello, Bob!
}
```

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

```rs
fn main() {
    let vec = vec![1, 2, 3];
    println!("{vec:?}");
    // [1, 2, 3]
}
```

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

```rs
fn main() {
    let point = (3, 4);
    let x = point.0;  // 訪問第一個元素
    let y = point.1; // 訪問第二個元素
    println!("{x}, {y}");
    // 3, 4
}
```

解構 (Destructuring):

```rs
fn main() {
    let point = (3, 4);
    let (x, y) = point;
    println!("{x}, {y}");
    // 3, 4
}
```

```rs
fn calculate_point() -> ((i32, i32), f64) {
    let (x, y) = (3, 4);
    let distance = ((x * x + y * y) as f64).sqrt();
    ((x, y), distance)
}

fn main() {
    let (point, distance) = calculate_point();
    println!("{:?}, {}", point, distance);
    // (3, 4), 5
}
```

## 結構 (Structures)

```rs

```

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

```rs
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    map.insert("Alice".into(), 60);
    map.insert("Bob".into(), 70);

    println!("{map:?}");
    // {"Alice": 60, "Bob": 70}
}
```

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

## 型別別名 (Type Alias)

```rs
type Age = u8;

fn print_age(age: Age) {
    println!("年齡是: {age}");
}

fn main() {
    let my_age: Age = 30;
    print_age(my_age);
    // 年齡是: 30
}
```

```rs
type Point = (f64, f64);

fn distance(p1: Point, p2: Point) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let point1: Point = (0.0, 0.0);
    let point2: Point = (3.0, 4.0);
    println!("兩點距離是: {}", distance(point1, point2));
    // 兩點距離是: 5
}
```
