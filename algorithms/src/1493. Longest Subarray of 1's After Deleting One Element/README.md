# 1493. (Longest Subarray of 1's After Deleting One Element)

給定一個二進制陣列 `nums`，你應該從中刪除一個元素。

回傳結果陣列中僅包含 `1` 的最長非空子陣列的長度。如果不存在這樣的子陣列，則返回 `0`。

範例 1：

```coffee
輸入: nums = [1,1,0,1]
輸出: 3
說明: 刪除位置 2 的數字後，[1,1,1] 包含 3 個 1。
```

範例 2：

```coffee
輸入: nums = [0,1,1,1,0,1,1,0,1]
輸出: 5
說明: 刪除位置 4 的數字後，[0,1,1,1,1,1,0,1] 的最長全 1 子陣列為 [1,1,1,1,1] 。
```

範例 3：

```coffee
輸入: nums = [1,1,1]
輸出: 2
說明: 你必須刪除一個元素。
```

## 解題

使用兩個指標 `left` 和 `right` 表示當前視窗。

```coffee
擴大
nums = [0,1,1,1,0,1,1,0,1]
       [l-------r]
zero_count = 2 (> 1)

縮小
nums = [0,1,1,1,0,1,1,0,1]
         [l-----r]
zero_count = 1 (符合條件)

繼續擴大
nums = [0,1,1,1,0,1,1,0,1]
         [l-----------r]
zero_count = 2 (> 1)
↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
在擴大中
nums = [0,1,1,1,0,1,1,0,1]
         [l---------r]
right - left = 6 - 1 = 5

再次縮小
nums = [0,1,1,1,0,1,1,0,1]
                 [l---r]
zero_count = 1 (符合條件)

繼續擴大
nums = [0,1,1,1,0,1,1,0,1]
                 [l-----r]

結束，最後的最長的非空子陣列長度為 5
```

```rs
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut zero_count = 0; // 當前視窗內的 0 的數量
        let mut max_len = 0; // 最長長度

        for right in 0..nums.len() {
            // 如果遇到 0，增加計數
            if nums[right] == 0 {
                zero_count += 1;
            }

            // 如果 0 的數量超過 1，縮小視窗
            while zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }

                left += 1;
            }

            // 更新最大長度 (刪除一個元素，所以減 1)
            max_len = max_len.max(right - left);
        }

        max_len as i32
    }
}
```
