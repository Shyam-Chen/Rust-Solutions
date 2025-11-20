struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        // 判斷是否為母音的輔助函式
        fn is_vowel(c: char) -> bool {
            matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
        }

        // 收集所有母音並反轉
        let vowels: Vec<_> = s.chars().filter(|&c| is_vowel(c)).rev().collect();

        // 用來依序取出反轉後的母音
        let mut iter = vowels.into_iter();

        // 映射所有字元，如果是母音，從迭代器取出下一個反轉後的母音，否則保持原字元
        s.chars()
            .map(|c| if is_vowel(c) { iter.next().unwrap() } else { c })
            .collect()
    }
}

#[cfg(test)]
#[path = "./reverse_vowels_test.rs"]
mod tests;
