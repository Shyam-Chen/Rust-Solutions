# 283. 移動零 (Move Zeroes)

給定一個整數陣列 `nums`，將所有 `0` 移至其末尾，同時保持非零元素的相對順序。

**注意**，你必須在不複製陣列的情況下原地對陣列進行操作。

範例 1：

```coffee
輸入: nums = [0,1,0,3,12]
輸出: [1,3,12,0,0]
```

範例 2：

```coffee
輸入: nums = [0]
輸出: [0]
```

## 解題

```rs
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero_index = 0; // 下一個非零元素應該放的位置

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(non_zero_index, i); // 把非零元素往前移，零自然被推到後
                non_zero_index += 1; // 更新指標，移動到下一個位置
            }
        }
    }
}
```
