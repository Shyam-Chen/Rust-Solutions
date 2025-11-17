struct Solution;

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

#[cfg(test)]
#[path = "./merge_alternately_test.rs"]
mod tests;
