struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut zero_count = 0; // 當前視窗內的 0 的數量
        let mut max_len = 0; // 最長長度

        for right in 0..nums.len() {
            // 如果遇到 0，增加計數
            if nums[right] == 0 {
                zero_count += 1;
            }

            // 如果 0 的數量超過 k，縮小視窗
            while zero_count > k {
                if nums[left] == 0 {
                    zero_count -= 1;
                }

                left += 1;
            }

            // 更新最大長度
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

#[cfg(test)]
#[path = "./longest_ones_test.rs"]
mod tests;
