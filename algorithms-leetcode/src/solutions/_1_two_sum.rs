use crate::solutions::Solution;

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
mod tests {
    use super::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
