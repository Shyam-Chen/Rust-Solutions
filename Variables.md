# 變數 (Variables)

```rs
fn main() {
    let val = 7;
    println!("值為：{val}");
    // 值為：7
}
```

### 可變的 (Mutable)

```rs
fn main() {
    let mut val = 7;
    println!("值為：{val}");
    // 值為：7

    val = 6;
    println!("值為：{val}");
    // 值為：6
}
```

# 常數 (Constants)

- 使用全大寫和底線分隔
- 型別必須明確指定

```rs
fn main() {
    const PI: f64 = 3.14159265;
    println!("{PI}");
    // 3.14159265

    const MAX_POINTS: u32 = 100_000;
    println!("{MAX_POINTS}");
    // 100000
}
```

# 遮蔽 (Shadowing)

```rs
fn main() {
    let val = 7;
    let val = val - 1;

    {
        let val = val * 2;
        println!("在內部作用域的值為：{val}");
        // 在作用域內的值為：12
    }

    println!("值為：{val}");
    // 值為：6
}
```
