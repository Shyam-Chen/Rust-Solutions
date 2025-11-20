# 392. 判斷子序列 (Is Subsequence)

給定兩個字串 `s` 和 `t`，如果 `s` 是 `t` 的子序列，則回傳 `true`，否則回傳 `false`。

字串的子序列是在不影響剩餘字元相對位置的情況下刪除原始字串中的一些 (可以是沒有) 字元而形成的新字串。(例如，`"ace"` 是 `"a̲bc̲de̲"` 的子序列，而 `"aec"` 不是)。

範例 1：

```coffee
輸入: s = "abc", t = "ahbgdc"
輸出: true
```

範例 2：

```coffee
輸入: s = "axc", t = "ahbgdc"
輸出: false
```

## 解題

```rs
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_vec: Vec<_> = s.chars().collect();
        let t_vec: Vec<_> = t.chars().collect();

        let mut i = 0; // 指向 s 的位置

        // 指向 t 的位置，由 for 迴圈隱式控制
        for &c in &t_vec {
            if i < s_vec.len() && s_vec[i] == c {
                i += 1;
            }
        }

        i == s_vec.len()
    }
}
```
