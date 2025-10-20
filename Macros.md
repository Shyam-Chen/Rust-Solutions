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

## 內建

### `todo!`

```rs
fn foo() {
    todo!();
}

fn bar() {
    todo!("This function needs to be implemented!");
}

fn main() {
    foo();
    bar();
}
```

### `file!`

```rs
fn main() {
    println!("File: {}", file!());
    // File: src/main.rs
}
```

### `cfg!`

```rs
fn main() {
    if cfg!(unix) {
        println!("Running on Unix!");
    } else {
        println!("Not running on Unix!");
    }
}
```

```rs
fn main() {
   if cfg!(target_os = "linux") {
       println!("Running on Linux!");
   } else {
       println!("Not running on Linux!");
   }
}
```
