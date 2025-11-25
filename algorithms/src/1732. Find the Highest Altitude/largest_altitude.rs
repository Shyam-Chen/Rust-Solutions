struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        // 前綴和陣列
        let mut prefix = Vec::with_capacity(gain.len() + 1);
        prefix.push(0);

        for &g in &gain {
            let last = *prefix.last().unwrap();
            prefix.push(last + g);
        }

        // 找出前綴和陣列中的最大值
        *prefix.iter().max().unwrap()
    }
}

#[cfg(test)]
#[path = "./largest_altitude_test.rs"]
mod tests;
