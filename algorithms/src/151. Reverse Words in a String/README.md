# 151. 反轉字串中的詞 (Reverse Words in a String)

給定一個輸入字串 `s`，反轉字串中**詞**的順序。

**詞**被定義為非空格字元的順序。`s` 中的**詞**將至少由一個空格分隔開。

回傳由單個空格按相反順序連接的詞彙。

注意，`s` 可能在兩個詞之間包含前導或尾隨空格或多個空格。回傳的字串應該只有用單個空格來分隔詞。請勿包含任何額外空格。

範例 1：

```coffee
輸入: s = "the sky is blue"
輸出: "blue is sky the"
```

範例 2：

```coffee
輸入: s = "  hello world  "
輸出: "world hello"
說明: 你的反轉字串不應包含前導或尾隨空格。
```

範例 3：

```coffee
輸入: s = "a good   example"
輸出: "example good a"
說明: 你需要將兩個詞之間的多個空格減少為反轉字串中的單個空格。
```

## 解題

```rs
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace() // 將字串依照空白分割，並自動忽略多餘空白 (包含前後空白)
            .rev() // 將分割後的迭代器反轉，讓最後一個詞變成第一個
            .collect::<Vec<_>>() // 將迭代器收集成 Vec<&str>，方便後續處理，元素型別由編譯器自動推斷
            .join(" ") // 將 Vec 中的詞用單一空白連接成一個字串
    }
}
```
