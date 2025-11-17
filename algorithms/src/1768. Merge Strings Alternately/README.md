# 1768. 交替合併字串 (Merge Strings Alternately)

給你兩個字串 `word1` 和 `word2`。請你從 `word1` 開始，透過交替加上字母來合併字串。如果一個字串比另一個字串長，就將多出來的字母追加到合併後字串的結尾。

返回*合併後的字串*。

範例 1：

```coffee
輸入: word1 = "abc", word2 = "pqr"
輸出: "apbqcr"
說明: 字串合併情況如下所示：
word1: a   b   c
word2:   p   q   r
合併後: a p b q c r
```

範例 2：

```coffee
輸入: word1 = "ab", word2 = "pqrs"
輸出: "apbqrs"
說明: 注意，word2 比 word1 長，"rs" 需要追加到合併後字串的結尾。
word1: a   b
word2:   p   q   r   s
合併後: a p b q   r   s
```

範例 3：

```coffee
輸入: word1 = "abcd", word2 = "pq"
輸出: "apbqcd"
說明: 注意，word1 比 word2 長，"cd" 需要追加到合併後字串的結尾。
word1: a   b   c   d
word2:   p   q
合併後: a p b q c   d
```

## 解題

### 使用迭代器，直到都取完字元：

```rs
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        loop {
            // 每次迴圈同時從兩個迭代器取出下一個字元
            match (chars1.next(), chars2.next()) {
                // 交替加入 result
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c1), None) => result.push(c1),
                (None, Some(c2)) => result.push(c2),
                // 兩個字串的所有字元都取完，跳出迴圈
                (None, None) => break,
            }
        }

        result
    }
}
```

### 使用 `flat_map` 方法，將兩個字串的字元交替合併：

```rs

```

### 使用雙指標 (Two Pointers)：

```rs

```
