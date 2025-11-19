use super::*;

#[test]
fn reverse_vowels() {
    assert_eq!(
        Solution::reverse_vowels(String::from("IceCreAm")),
        "AceCreIm"
    );
    assert_eq!(
        Solution::reverse_vowels(String::from("leetcode")),
        "leotcede"
    );
}
