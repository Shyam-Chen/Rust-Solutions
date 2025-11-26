use super::*;

#[test]
fn pivot_index() {
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
}
