use super::*;

#[test]
fn max_vowels() {
    assert_eq!(Solution::max_vowels(String::from("abciiidef"), 3), 3);
    assert_eq!(Solution::max_vowels(String::from("aeiou"), 2), 2);
    assert_eq!(Solution::max_vowels(String::from("leetcode"), 3), 2);
}
