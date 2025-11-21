# 1071. 字串的最大公因數 (Greatest Common Divisor of Strings)

對於兩個字串 `s` 和 `t`，只有在 `s = t + t + t + ... + t + t` (即 `t` 與自身連接一次或多次) 時，我們才認定「`t` 可以整除 `s`」。

給定兩個字串 `str1` 和 `str2`，回傳最長的字串 `x`，使得 `x` 可以同時整除 `str1` 和 `str2`。

範例 1：

```coffee
輸入: str1 = "ABCABC", str2 = "ABC"
輸出: "ABC"
```

範例 2：

```coffee
輸入: str1 = "ABABAB", str2 = "ABAB"
輸出: "AB"
```

範例 3：

```coffee
輸入: str1 = "LEET", str2 = "CODE"
輸出: ""
```

## 解題

```rs
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // 判斷是否有字串的最大公因數
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return String::new();
        }

        // 計算長度的最大公因數 (GCD)
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        // 取出字串的最大公因數
        let len = gcd(str1.len(), str2.len());
        str1[..len].to_string()
    }
}
```

判斷是否有字串的最大公因數：

```rs
if str1.clone() + &str2 != str2.clone() + &str1 {
    return String::new();
}
```

假設存在一個字串 `x`，它能整除 `str1` 和 `str2`，也就是：

```rs
str1 = x 重複 a 次
str2 = x 重複 b 次
```

那麼：

```rs
str1 + str2 = x 重複 (a + b) 次
str2 + str1 = x 重複 (b + a) 次
```

兩者必然相等，因為 `(a + b) = (b + a)`。

也就是說，如果 `str1 + str2 != str2 + str1`，就代表：

無法找到一個共同的模式 `x`，讓兩個字串都是它的重複，也因此沒有「字串的最大公因數」，直接回傳空字串。

這檢查是一個**快速過濾條件**，避免進一步計算 GCD 時浪費時間。

計算長度的最大公因數 (GCD)：

```rs
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}
```

為輾轉相除法，又稱歐幾里得演算法 (Euclidean Algorithm)，是求最大公因數的演算法。
