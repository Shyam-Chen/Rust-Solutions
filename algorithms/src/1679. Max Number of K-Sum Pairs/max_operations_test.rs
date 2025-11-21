use super::*;

#[test]
fn max_operations() {
    assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
}
