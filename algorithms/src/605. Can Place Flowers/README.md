# 605. 種花問題 (Can Place Flowers)

假設有一個很長的花壇，一部分地塊種了花，另一部分卻沒有。可是，花不能種植在**相鄰的**地塊上，它們會爭奪水源，兩者都會死。

給定一個整數陣列 `flowerbed` 表示花壇，由若干 `0` 和 `1` 組成，其中 `0` 表示沒種植花，`1` 表示種植了花，另有一個整數 `n`，能否在不打破種植規則的情況下種入 `n` 朵花？能則回傳 `true`，不能則回傳 `false`。

範例 1：

```coffee
輸入: flowerbed = [1,0,0,0,1], n = 1
輸出: true
```

範例 2：

```coffee
輸入: flowerbed = [1,0,0,0,1], n = 2
輸出: false
```

## 解題

```rs
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
```
