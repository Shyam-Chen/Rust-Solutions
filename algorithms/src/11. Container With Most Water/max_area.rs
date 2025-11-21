struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // 初始化雙指標
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut max_area = 0;

        // 當左指標小於右指標時，持續計算
        while left < right {
            // 計算當前高度、寬度和面積
            let current_height = height[left].min(height[right]);
            let current_width = (right - left) as i32;
            let current_area = current_height * current_width;

            // 更新最大面積
            if current_area > max_area {
                max_area = current_area;
            }

            // 移動指標，移動較短的那一邊
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
#[path = "./max_area_test.rs"]
mod tests;
