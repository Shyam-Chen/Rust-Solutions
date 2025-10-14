use crate::solutions::_1768_merge_alternately::Solution;

#[test]
fn test_merge_alternately() {
    println!("123");
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
