use super::*;

#[test]
fn kids_with_candies() {
    assert_eq!(
        Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
        [true, true, true, false, true]
    );
    assert_eq!(
        Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
        [true, false, false, false, false]
    );
    assert_eq!(
        Solution::kids_with_candies(vec![12, 1, 12], 10),
        [true, false, true]
    );
}
