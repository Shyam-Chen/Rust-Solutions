struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;

        // 計算前 k 個元素的總和，做為初始視窗
        let mut window_sum: i32 = nums[..k].iter().sum();

        // 目前為止看到的最大視窗總和
        let mut max_sum = window_sum;

        // 開始滑動視窗 (從索引 k 開始)
        for i in k..nums.len() {
            // 滑出左邊元素，滑入右邊元素
            window_sum += nums[i] - nums[i - k];

            // 更新最大視窗總和
            if window_sum > max_sum {
                max_sum = window_sum;
            }
        }

        // 回傳最大平均值
        max_sum as f64 / k as f64
    }
}

#[cfg(test)]
#[path = "./find_max_average_test.rs"]
mod tests;
