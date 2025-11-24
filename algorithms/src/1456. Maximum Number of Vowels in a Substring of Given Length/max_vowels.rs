struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;

        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let chars: Vec<char> = s.chars().collect();

        // 初始視窗
        let mut count = chars[..k].iter().filter(|c| vowels.contains(c)).count();
        let mut max_count = count;

        // 滑動視窗
        for i in k..chars.len() {
            // 移除舊字元
            if vowels.contains(&chars[i - k]) {
                count -= 1;
            }

            // 加入新字元
            if vowels.contains(&chars[i]) {
                count += 1;
            }

            // 更新最大值
            if count > max_count {
                max_count = count;
            }
        }

        max_count as i32
    }
}

#[cfg(test)]
#[path = "./max_vowels_test.rs"]
mod tests;
