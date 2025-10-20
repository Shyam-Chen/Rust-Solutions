# 數學 (Math)

```rs
fn main() {
    let a = 7;
    let b = 3;
    println!("加法: {a} + {b} = {v}", v = a + b);
    println!("減法: {a} - {b} = {v}", v = a - b);
    println!("乘法: {a} * {b} = {v}", v = a * b);
    println!("除法: {a} / {b} = {v}", v = a / b);
    println!("取餘數: {a} % {b} = {v}", v = a % b);
}
// 加法: 7 + 3 = 10
// 減法: 7 - 3 = 4
// 乘法: 7 * 3 = 21
// 除法: 7 / 3 = 2
// 取餘數: 7 % 3 = 1
```

```sh
$ cargo add rand
```

```rs
fn main() {
    let val: u8 = rand::random();
    println!("{val}");
}
```

```sh
$ cargo add num
```

```rs
use num::BigInt;
```
