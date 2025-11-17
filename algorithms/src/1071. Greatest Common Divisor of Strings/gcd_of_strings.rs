struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // 判斷是否有字串的最大公因數
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return String::new();
        }

        // 計算長度的最大公因數 (GCD)
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        // 取出字串的最大公因數
        let len = gcd(str1.len(), str2.len());
        str1[..len].to_string()
    }
}

#[cfg(test)]
#[path = "./gcd_of_strings_test.rs"]
mod tests;
