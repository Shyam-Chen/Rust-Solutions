# 函式 (Functions)

```rs
fn greet() {
    println!("Hello, World!");
}

fn main() {
    greet();
    greet_person("Shyam");
}

fn greet_person(name: &str) {
    println!("Hello, {name}!");
}
```

### 表達式 (Expressions)

```rs
fn main() {
    let val = {
        let a = 5;
        let b = 3;
        a + b // 沒有分號
    };

    println!("值為：{val}");
    // 值為：8
}
```

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b // 沒有分號表示這是表達式，會被當作返回值
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {result}");
    // 5 + 3 = 8

    let result = sub(5, 3);
    println!("5 - 3 = {result}");
    // 5 - 3 = 2
}
```

### 閉包 (Closures)

```rs
fn main() {
    // 函式定義
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    // 閉包定義
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 }; // Rustfmt 執行後將變為 v4 的樣式
    let add_one_v4 = |x| x + 1;

    let a = add_one_v1(6);
    let b = add_one_v2(6);
    let c = add_one_v3(6);
    let d = add_one_v4(6);

    println!("{a}, {b}, {c}, {d}");
    // 7, 7, 7, 7
}
```

## 實作 impl

## 特徵 trait
