struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();

        let mut read = 0; // 讀取指標，原始讀取的位置
        let mut write = 0; // 寫入指標，壓縮後寫入的位置

        while read < n {
            let current_char = chars[read];
            let mut count = 0;

            // 計算連續相同字元的數量
            while read < n && chars[read] == current_char {
                read += 1;
                count += 1;
            }

            // 寫入該字元
            chars[write] = current_char;
            write += 1;

            // 如果 count > 1，寫入數字
            if count > 1 {
                for c in count.to_string().chars() {
                    chars[write] = c;
                    write += 1;
                }
            }
        }

        write as i32 // 回傳壓縮後的長度
    }
}

#[cfg(test)]
#[path = "./compress_test.rs"]
mod tests;
