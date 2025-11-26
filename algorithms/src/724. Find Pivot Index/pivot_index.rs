struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // 先計算整個陣列的總和
        let total_sum: i32 = nums.iter().sum();

        // 初始化左側和
        let mut left_sum = 0;

        for (i, &num) in nums.iter().enumerate() {
            // 右側和 = total_sum - left_sum - num
            if left_sum == total_sum - left_sum - num {
                return i as i32; // 找到樞軸索引
            }

            // 更新左側和
            left_sum += num;
        }

        // 沒有找到，回傳 -1
        -1
    }
}

#[cfg(test)]
#[path = "./pivot_index_test.rs"]
mod tests;
