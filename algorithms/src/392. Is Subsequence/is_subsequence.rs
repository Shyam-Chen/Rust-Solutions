struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_vec: Vec<_> = s.chars().collect();
        let t_vec: Vec<_> = t.chars().collect();

        let mut i = 0; // 指向 s 的位置

        // 指向 t 的位置，由 for 迴圈隱式控制
        for &c in &t_vec {
            if i < s_vec.len() && s_vec[i] == c {
                i += 1;
            }
        }

        i == s_vec.len()
    }
}

#[cfg(test)]
#[path = "./is_subsequence_test.rs"]
mod tests;
