use super::*;

#[test]
fn find_max_average() {
    assert_eq!(
        Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
        12.75000
    );
    assert_eq!(Solution::find_max_average(vec![5], 1), 5.00000);
}
