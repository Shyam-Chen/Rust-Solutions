# 測試 (Testing)

```rs
// src/math.rs
#![allow(dead_code)]

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn div(a: i32, b: i32) -> i32 {
    a / b
}

#[cfg(test)]
mod math_tests {
    // 將測試模組置於 super 命名空間，讓我們可以呼叫外部定義的函式
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5); // 測試成功條件：2 + 3 = 5
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5, 3), 2); // 測試成功條件：5 - 3 = 2
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(4, 3), 12); // 測試成功條件：4 * 3 = 12
    }

    #[test]
    fn test_div() {
        assert_eq!(div(10, 2), 5); // 測試成功條件：10 / 2 = 5
    }

    #[test]
    #[should_panic]
    fn test_div_panic() {
        div(5, 0); // 此操作會 panic
    }
}
```
