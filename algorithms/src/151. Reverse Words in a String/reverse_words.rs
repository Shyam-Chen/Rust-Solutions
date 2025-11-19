struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace() // 將字串依照空白分割，並自動忽略多餘空白 (包含前後空白)
            .rev() // 將分割後的迭代器反轉，讓最後一個詞變成第一個
            .collect::<Vec<_>>() // 將迭代器收集成 Vec<&str>，方便後續處理，元素型別由編譯器自動推斷
            .join(" ") // 將 Vec 中的詞用單一空白連接成一個字串
    }
}

#[cfg(test)]
#[path = "./reverse_words_test.rs"]
mod tests;
