struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0; // 計算已經種下的花數量
        let mut i = 0; // 當前索引位置

        let len = flowerbed.len();

        while i < len {
            // 如果當前位置是空的 (0)
            if flowerbed[i] == 0 {
                // 檢查左邊是否空，或是否在邊界
                let prev_empty = i == 0 || flowerbed[i - 1] == 0;

                // 檢查右邊是否空，或是否在邊界
                let next_empty = i == len - 1 || flowerbed[i + 1] == 0;

                // 如果左右都空，可以種花
                if prev_empty && next_empty {
                    count += 1; // 種花後計數 +1

                    // 如果已種夠 n 朵花，直接回傳 true
                    if count >= n {
                        return true;
                    }

                    i += 2; // 跳過下一格，避免相鄰種花
                    continue; // 進入下一次迴圈
                }
            }

            i += 1; // 如果不能種花，或當前位置是 1，移動到下一格
        }

        count >= n // 走訪完成後，檢查是否種夠花
    }
}

#[cfg(test)]
#[path = "./can_place_flowers_test.rs"]
mod tests;
