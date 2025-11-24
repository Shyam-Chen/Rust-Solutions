# 1004. 最大連續 1 的個數 III (Max Consecutive Ones III)

給定一個二進制陣列 `nums` 和一個整數 `k`，如果最多可以翻轉 `k` 個 `0`，則回傳陣列中連續 `1` 的最大個數。

範例 1：

```coffee
輸入: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
輸出: 6
說明:
[1,1,1,0,0,0,1,1,1,1,0]
           ↓         ↓ 從 0 翻轉到 1
[1,1,1,0,0,1,1,1,1,1,1]
           ●---------● 最長的子陣列長度為 6
```

範例 2：

```coffee
輸入: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
輸出: 10
說明:
[0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1]
         ↓ ↓       ↓                   從 0 翻轉到 1
[0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
     ●-----------------●               最長的子陣列長度為 10
```

## 解題

使用兩個指標 `left` 和 `right` 表示當前視窗。

```coffee
擴大
nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
       [l---------r]
zero_count = 3 (> k)

縮小
nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
               [l-r]
zero_count = 2 (符合條件)

繼續擴大
nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
               [l-----------r]
zero_count = 3 (> k)

再次縮小
nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
                 [l---------r]
zero_count = 2 (符合條件)

結束，最後的最長的子陣列長度為 6
```

```rs
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut zero_count = 0; // 當前視窗內的 0 的數量
        let mut max_len = 0; // 最長長度

        for right in 0..nums.len() {
            // 如果遇到 0，增加計數
            if nums[right] == 0 {
                zero_count += 1;
            }

            // 如果 0 的數量超過 k，縮小視窗
            while zero_count > k {
                if nums[left] == 0 {
                    zero_count -= 1;
                }

                left += 1;
            }

            // 更新最大長度
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
```
