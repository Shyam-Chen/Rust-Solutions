struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        // 初始化 first 和 second 為最大值
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for num in nums {
            if num <= first {
                // 更新第一個最小值
                first = num;
            } else if num <= second {
                // 更新第二個值 (比 first 大)
                second = num;
            } else {
                // num > second，代表找到第三個數
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
#[path = "./increasing_triplet_test.rs"]
mod tests;
