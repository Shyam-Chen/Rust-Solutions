# 334. 遞增的三元子序列 (Increasing Triplet Subsequence)

給定一個整數陣列 `nums`，如果存在這樣的三元組索引 `(i, j, k)` 且滿足條件 `i < j < k`，使得 `nums[i] < nums[j] < nums[k]`，則回傳 `true`。如果不存在這樣的索引，則回傳 `false`。

範例 1：

```coffee
輸入: nums = [1,2,3,4,5]
輸出: true
說明: 任何 i < j < k 的三元組索引都滿足條件。
```

範例 2：

```coffee
輸入: nums = [5,4,3,2,1]
輸出: false
說明: 不存在滿足條件的三元組索引。
```

範例 3：

```coffee
輸入: nums = [2,1,5,0,4,6]
輸出: true
說明: 其中一個滿足條件的三元組索引是 (1, 4, 5)，因為 nums[1] == 1 < nums[4] == 4 < nums[5] == 6。
```

## 解題

```rs
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        // 初始化 first 和 second 為最大值
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for num in nums {
            if num <= first {
                // 更新第一個最小值
                first = num;
            } else if num <= second {
                // 更新第二個值 (比 first 大)
                second = num;
            } else {
                // num > second，代表找到第三個數
                return true;
            }
        }

        false
    }
}
```
