struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // 先計算 candies 陣列中目前的最大糖果數
        let max_candies = *candies.iter().max().unwrap();

        // 對每個小孩，檢查 candies[i] + extraCandies 是否大於或等於最大值。
        candies
            .into_iter()
            .map(|candy| candy + extra_candies >= max_candies)
            .collect()
    }
}

#[cfg(test)]
#[path = "./kids_with_candies_test.rs"]
mod tests;
