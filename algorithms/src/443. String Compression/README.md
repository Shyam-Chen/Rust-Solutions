# 443. 壓縮字串 (String Compression)

給定一個字元陣列 `chars`，使用以下敘述的演算法對其進行壓縮：

從一個空字串 `s` 開始。對於 `chars` 中的每組**連續重複字元**：

- 如果這一組長度為 `1`，則將字元追加到 `s` 中。
- 否則，需要向 `s` 追加字元，後面跟著這一組的長度。

壓縮後得到的字串 `s` **不應該直接回傳**，而是轉儲存在**輸入字元陣列** `chars` 中。需要注意的是，如果該組長度為 `10` 或 `10` 以上，則在 `chars` 陣列中會被拆分為多個字元。

**修改輸入陣列**完後，回傳該陣列的新長度。

你必須寫一個僅使用固定大小額外空間的演算法。

**注意**：陣列中超出回傳長度的字元無關緊要，應予忽略。

範例 1：

```coffee
輸入: chars = ["a","a","b","b","c","c","c"]
輸出: 回傳 6，輸入陣列的前 6 個字元應該是：["a","2","b","2","c","3"]
說明: 這些組是 "aa"、"bb" 和 "ccc"。這會壓縮為 "a2b2c3"。
```

範例 2：

```coffee
輸入: chars = ["a"]
輸出: 回傳 1，輸入陣列的前 1 個字元應該是：["a"]
說明: 唯一的組是 "a"，它保持未壓縮，因為它是一個字元。
```

範例 3：

```coffee
輸入: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
輸出: 回傳 4，輸入陣列的前 4 個字元應該是：["a","b","1","2"]。
說明: 這些組是 "a" 和 "bbbbbbbbbbbb"。這會壓縮為 "ab12"。
```

## 解題

即為：

```coffee
chars = ["a","a","b","b","c","c","c"]
"aa" → "a2"
"bb" → "b2"
"ccc" → "c3"

chars = ["a"]
"a" → "a"，不會是 "a1"，所以必須大於 1 才可寫入

chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
"a" → "a"，一樣不會是 "a1"，所以也是必須大於 1 才可寫入
"bbbbbbbbbbbb" → "b12"
```

```rs
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();

        let mut read = 0; // 讀取指標，原始讀取的位置
        let mut write = 0; // 寫入指標，壓縮後寫入的位置

        while read < n {
            let current_char = chars[read];
            let mut count = 0;

            // 計算連續相同字元的數量
            while read < n && chars[read] == current_char {
                read += 1;
                count += 1;
            }

            // 寫入該字元
            chars[write] = current_char;
            write += 1;

            // 如果 count > 1，寫入數字
            if count > 1 {
                for c in count.to_string().chars() {
                    chars[write] = c;
                    write += 1;
                }
            }
        }

        write as i32 // 回傳壓縮後的長度
    }
}
```
