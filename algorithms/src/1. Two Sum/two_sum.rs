struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            // 查找是否存在另外一個數值符合相加為 target
            let complement = target - num;

            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }

            // 將當前數字與其索引存入 HashMap，供後續查找
            map.insert(num, i);
        }

        vec![] // 若未找到解則回傳空陣列
    }
}

#[cfg(test)]
#[path = "./two_sum_test.rs"]
mod tests;
