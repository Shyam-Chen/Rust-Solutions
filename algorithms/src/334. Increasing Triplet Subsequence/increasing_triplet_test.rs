use super::*;

#[test]
fn increasing_triplet() {
    assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
}
