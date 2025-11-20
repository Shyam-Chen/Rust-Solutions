struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero_index = 0; // 下一個非零元素應該放的位置

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(non_zero_index, i); // 把非零元素往前移，零自然被推到後
                non_zero_index += 1; // 更新指標，移動到下一個位置
            }
        }
    }
}

#[cfg(test)]
#[path = "./move_zeroes_test.rs"]
mod tests;
