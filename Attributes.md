# 屬性 (Attributes)

## 內建屬性

### `#[derive]`

### `#[allow]` 和 `#[deny]`

全域: `#![allow(dead_code)]`

```rs
#![allow(dead_code)]

fn unused_function() {
    println!("This function is never used");
}

struct UnusedStruct {
    value: i32,
}

fn main() {
    println!("No warnings will be displayed.");
}
```

局部: `#[allow(dead_code)]`

```rs
#[allow(dead_code)]
fn unused_function() {
    println!("This function is never used");
}

#[allow(dead_code)]
struct UnusedStruct {
    value: i32,
}

fn main() {
    println!("No warnings will be displayed.");
}
```

### `#[inline]`

### `#[cfg]` 和 `#[cfg_attr]`

### `#[non_exhaustive]`

## 自訂屬性 (Custom Attributes (程序式巨集, Procedural Macros))
