struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1; n];

        // 累積左邊的乘積
        let mut left_product = 1;

        for i in 0..n {
            answer[i] = left_product; // 將目前的左乘積存入 answer[i]
            left_product *= nums[i]; // 更新左乘積，乘上當前元素
        }

        // 累積右邊的乘積
        let mut right_product = 1;

        for i in (0..n).rev() {
            answer[i] *= right_product; // 將右乘積乘到 answer[i]，此時 answer[i] 已經有左乘積
            right_product *= nums[i]; // 更新右乘積，乘上當前元素
        }

        answer
    }
}

#[cfg(test)]
#[path = "./product_except_self_test.rs"]
mod tests;
