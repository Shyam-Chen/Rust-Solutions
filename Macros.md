# 巨集 (Macros)

```rs
macro_rules! hello_world {
    () => {
        println!("Hello, World!");
    };
}

fn main() {
    hello_world!();
    // Hello, World!
}
```

## 帶參數 (With Parameters)

### 表達式: `$val:expr`

```rs
macro_rules! hello {
    ($text:expr) => {
        println!("Hello, {}!", $text);
    };
}

fn main() {
    hello!("World");
    // Hello, World!
}
```
