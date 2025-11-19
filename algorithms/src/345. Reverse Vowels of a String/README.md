# 345. 反轉字串中的母音 (Reverse Vowels of a String)

給定一個字串 `s`，僅反轉字串中的所有母音並回傳它。

母音為 `'a'`、`'e'`、`'i'`、`'o'` 和 `'u'`，且可以以大小寫形式出現不止一次。

範例 1：

```coffee
輸入: s = "IceCreAm"
輸出: "AceCreIm"
說明: s 中的母音是 ['I', 'e', 'e', 'A']。反轉這些母音，s 變成 "AceCreIm"。
```

範例 2：

```coffee
輸入: s = "leetcode"
輸出: "leotcede"
```

## 解題

```rs
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        // 判斷是否為母音的輔助函式
        fn is_vowel(c: char) -> bool {
            matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
        }

        // 收集所有母音並反轉
        let vowels: Vec<char> = s.chars().filter(|&c| is_vowel(c)).rev().collect();

        // 用來依序取出反轉後的母音
        let mut iter = vowels.into_iter();

        // 映射所有字元，如果是母音，從迭代器取出下一個反轉後的母音，否則保持原字元
        s.chars()
            .map(|c| if is_vowel(c) { iter.next().unwrap() } else { c })
            .collect()
    }
}
```
