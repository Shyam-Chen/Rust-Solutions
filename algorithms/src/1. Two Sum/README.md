# 1. 兩數之和 (Two Sum)

給定一個整數陣列 `nums` 和一個整數目標值 `target`，返回兩個數字的索引，使它們相加等於目標值 `target`。

你可以假設每個輸入只會**對應一個答案**，而且你不能使用兩次相同的元素。

你可以按任意順序返回答案。

範例 1：

```coffee
輸入: nums = [2,7,11,15], target = 9
輸出: [0,1]
說明: 因為 nums[0] + nums[1] == 9，返回 [0, 1]。
```

範例 2：

```coffee
輸入: nums = [3,2,4], target = 6
輸出: [1,2]
```

範例 3：

```coffee
輸入: nums = [3,3], target = 6
輸出: [0,1]
```

## 解題

### 使用雜湊表 (Hash Table)

```rs
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            // 查找是否存在另外一個數值符合相加為 target
            let complement = target - num;

            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }

            // 將當前數字與其索引存入 HashMap，供後續查找
            map.insert(num, i);
        }

        vec![] // 若未找到解則回傳空陣列
    }
}
```

### 暴力解法

```rs

```
