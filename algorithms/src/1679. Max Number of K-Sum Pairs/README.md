# 1679. K 和對的最大數目 (Max Number of K-Sum Pairs)

給定一個整數陣列 `nums` 和一個整數 `k`。

每一次操作中，你可以從陣列中選取兩個總和等於 `k` 的數字並將其從陣列中刪除。

回傳你可以對陣列執行的最大操作數。

範例 1：

```coffee
輸入: nums = [1,2,3,4], k = 5
輸出: 2
說明: 開始時 nums = [1,2,3,4]:
- 移出 1 和 4，之後 nums = [2,3]
- 移出 2 和 3，之後 nums = []
不再有和為 5 的數對，因此執行最多 2 次操作。
```

範例 2：

```coffee
輸入: nums = [3,1,3,4,3], k = 6
輸出: 1
說明: 開始時 nums = [3,1,3,4,3]:
- 移出前兩個 3，之後 nums = [1,4,3]
不再有和為 6 的數對，因此最多執行 1 次操作。
```

## 解題

### 先排序，再使用雙指標

```coffee
以 nums = [1,2,3,4], k = 5 來看:

l
↓
1 - 2 - 3 - 4
            ↑
            r

1 + 4 = 5，count + 1，下個迴圈

    l
    ↓
1 - 2 - 3 - 4
        ↑
        r

2 + 3 = 5，count + 1，left + 1，right - 1，left < right，迴圈結束
```

```coffee
以 nums = [3,1,3,4,3], k = 6 來看，nums 排序後 [1,3,3,3,4]:

l
↓
1 - 3 - 3 - 3 - 4
                ↑
                r

1 + 4 < 6，總和太小，left + 1，下個迴圈

    l
    ↓
1 - 3 - 3 - 3 - 4
                ↑
                r

3 + 4 > 6，總和太大，right - 1，下個迴圈

    l
    ↓
1 - 3 - 3 - 3 - 4
            ↑
            r

3 + 3 = 6，count + 1，left + 1，right - 1，left < right，迴圈結束
```

```rs
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut arr = nums.clone();
        arr.sort(); // 先排序

        let mut left = 0;
        let mut right = arr.len() - 1;
        let mut count = 0;

        while left < right {
            let sum = arr[left] + arr[right];

            if sum == k {
                count += 1;
                left += 1;
                right -= 1;
            } else if sum < k {
                left += 1; // 總和太小，往右移動
            } else {
                right -= 1; // 總和太大，往左移動
            }
        }

        count
    }
}
```

### 使用雜湊表

```rs
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new(); // 用來記錄每個數字出現的次數
        let mut count = 0;

        for num in nums {
            let complement = k - num; // 需要的另一個數字

            if let Some(&freq) = map.get(&complement) {
                if freq > 0 {
                    // 找到一個配對
                    count += 1;
                    map.insert(complement, freq - 1); // 減少 complement 的次數
                    continue;
                }
            }

            // 如果沒有找到配對，記錄 num 的次數
            *map.entry(num).or_insert(0) += 1;
        }

        count
    }
}
```
