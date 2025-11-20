use super::*;

#[test]
fn is_subsequence() {
    assert_eq!(
        Solution::is_subsequence(String::from("abc"), String::from("ahbgdc")),
        true
    );
    assert_eq!(
        Solution::is_subsequence(String::from("axc"), String::from("ahbgdc")),
        false
    );
}
