use super::*;

#[test]
fn product_except_self() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        [24, 12, 8, 6]
    );
    assert_eq!(
        Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
        [0, 0, 9, 0, 0]
    );
}
