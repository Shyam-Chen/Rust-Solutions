use super::*;

#[test]
fn longest_ones() {
    assert_eq!(
        Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
        6
    );
    assert_eq!(
        Solution::longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3
        ),
        10
    );
}
