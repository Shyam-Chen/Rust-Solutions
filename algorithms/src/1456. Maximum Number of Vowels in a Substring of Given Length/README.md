# 1456. 給定長度的子字串中母音的最大數目 (Maximum Number of Vowels in a Substring of Given Length)

給定一個字串 `s` 和一個整數 `k`，回傳 `s` 中長度為 `k` 的任何子字串中母音字母的最大數目。

英文中的**母音字母**為 `'a'`、`'e'`、`'i'`、`'o'` 和 `'u'`。

範例 1：

```coffee
輸入: s = "abciiidef", k = 3
輸出: 3
說明: 子字串 "iii" 包含 3 個母音字母。
```

範例 2：

```coffee
輸入: s = "aeiou", k = 2
輸出: 2
說明: 任意長度為 2 的子字串都包含 2 個母音字母。
```

範例 3：

```coffee
輸入: s = "leetcode", k = 3
輸出: 2
說明: "lee"、"eet" 和 "ode" 都包含 2 個母音字母。
```

## 解題

```rs
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;

        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let chars: Vec<char> = s.chars().collect();

        // 初始視窗
        let mut count = chars[..k].iter().filter(|c| vowels.contains(c)).count();
        let mut max_count = count;

        // 滑動視窗
        for i in k..chars.len() {
            // 移除舊字元
            if vowels.contains(&chars[i - k]) {
                count -= 1;
            }

            // 加入新字元
            if vowels.contains(&chars[i]) {
                count += 1;
            }

            // 更新最大值
            if count > max_count {
                max_count = count;
            }
        }

        max_count as i32
    }
}
```
