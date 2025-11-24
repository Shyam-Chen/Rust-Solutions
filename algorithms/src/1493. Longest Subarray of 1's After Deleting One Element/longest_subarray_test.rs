use super::*;

#[test]
fn longest_subarray() {
    assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
    assert_eq!(
        Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
        5
    );
    assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
}
