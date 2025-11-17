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

```rs
use std::cmp::{max, min};

fn main() {
    println!("{}", max(1, 3)); // 3
    println!("{}", min(1, 3)); // 1
}
```

```rs
fn main() {
    let numbers = vec![1, 3, 2, 5, 4];
    let my_num = 3;

    let max_num = numbers.iter().max().unwrap();
    println!("{max_num}"); // 5
    println!("{}", my_num < *max_num); // true

    let min_num = numbers.iter().min().unwrap();
    println!("{min_num}"); // 1
    println!("{}", my_num > *min_num); // true
}
```

```sh
$ cargo add rand
```

```rs
fn main() {
    let val: u8 = rand::random(); // 0 ~ 255 隨機數
    println!("{val}");

    println!("{}", rand::random::<u8>()); // 使用泛型

    println!("{}", rand::random_range(1..=10)); // 1 ~ 10 隨機數
}
```

```sh
$ cargo add num
```

```rs
use num::BigInt;
```
