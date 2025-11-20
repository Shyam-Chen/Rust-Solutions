use super::*;

#[test]
fn move_zeroes() {
    {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, [1, 3, 12, 0, 0]);
    }

    {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, [0]);
    }
}
