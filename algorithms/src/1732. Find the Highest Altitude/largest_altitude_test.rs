use super::*;

#[test]
fn largest_altitude() {
    assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
}
