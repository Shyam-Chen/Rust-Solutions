# 724. 尋找樞軸索引 (Find Pivot Index)

給定一個整數陣列 `nums`，計算該陣列的**樞軸索引**。

**樞軸索引**是個陣列索引，其左側的所有數字之和等於右側的所有數字之和。

如果索引位於陣列的最左端，那麼左側數總和視為 `0`，因為在索引的左側不存在元素。這點同樣也適用於索引位於陣列的最右端。

回傳**最左端的樞軸索引**。如果不存在這樣的索引，則回傳 `-1`。

範例 1：

```coffee
輸入: nums = [1,7,3,6,5,6]
輸出: 3
說明:
樞軸索引是 3。
左側和 = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11
右側和 = nums[4] + nums[5] = 5 + 6 = 11
```

範例 2：

```coffee
輸入: nums = [1,2,3]
輸出: -1
說明: 不存在滿足此條件的樞軸索引。
```

範例 3：

```coffee
輸入: nums = [2,1,-1]
輸出: 0
說明:
左側和 = 0 (索引 0 左側不存在元素)
右側和= nums[1] + nums[2] = 1 + -1 = 0
```

## 解題

```rs
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // 先計算整個陣列的總和
        let total_sum: i32 = nums.iter().sum();

        // 初始化左側和
        let mut left_sum = 0;

        for (i, &num) in nums.iter().enumerate() {
            // 右側和 = total_sum - left_sum - num
            if left_sum == total_sum - left_sum - num {
                return i as i32; // 找到樞軸索引
            }

            // 更新左側和
            left_sum += num;
        }

        // 沒有找到，回傳 -1
        -1
    }
}
```
