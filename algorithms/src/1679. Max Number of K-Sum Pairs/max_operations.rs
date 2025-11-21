struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut arr = nums.clone();
        arr.sort(); // 先排序

        let mut left = 0;
        let mut right = arr.len() - 1;
        let mut count = 0;

        while left < right {
            let sum = arr[left] + arr[right];

            if sum == k {
                count += 1;
                left += 1;
                right -= 1;
            } else if sum < k {
                left += 1; // 總和太小，往右移動
            } else {
                right -= 1; // 總和太大，往左移動
            }
        }

        count
    }
}

#[cfg(test)]
#[path = "./max_operations_test.rs"]
mod tests;
