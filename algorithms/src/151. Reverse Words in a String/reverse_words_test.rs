use super::*;

#[test]
fn reverse_words() {
    assert_eq!(
        Solution::reverse_words(String::from("the sky is blue")),
        "blue is sky the"
    );
    assert_eq!(
        Solution::reverse_words(String::from("  hello world  ")),
        "world hello"
    );
    assert_eq!(
        Solution::reverse_words(String::from("a good   example")),
        "example good a"
    );
}
