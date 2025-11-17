use super::*;

#[test]
fn merge_alternately() {
    assert_eq!(
        Solution::merge_alternately(String::from("abc"), String::from("pqr")),
        "apbqcr"
    );
    assert_eq!(
        Solution::merge_alternately(String::from("ab"), String::from("pqrs")),
        "apbqrs"
    );
    assert_eq!(
        Solution::merge_alternately(String::from("abcd"), String::from("pq")),
        "apbqcd"
    );
}
