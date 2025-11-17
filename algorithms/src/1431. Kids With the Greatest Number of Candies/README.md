# 擁有最多糖果的孩子 (Kids With the Greatest Number of Candies)

有 `n` 個有糖果的孩子。給你一個陣列 `candies`，其中 `candies[i]` 代表第 `i` 個孩子擁有的糖果數目，和一個整數 `extraCandies` 表示你所有的額外糖果的數量。

回傳一個長度為 `n` 的布林陣列 `result`，如果把所有的 `extraCandies` 給第 `i` 個孩子之後，他會擁有所有孩子中**最多**的糖果，那麼 `result[i]` 為 `true`，否則為 `false`。

注意，允許有多個孩子同時擁有**最多**的糖果數目。

範例 1：

```coffee
輸入: candies = [2,3,5,1,3], extraCandies = 3
輸出: [true,true,true,false,true]
說明: 如果你把額外的糖果全部給:
- 孩子 1，將有 2 + 3 = 5 顆糖果，是孩子中最多的。
- 孩子 2，將有 3 + 3 = 6 顆糖果，是孩子中最多的。
- 孩子 3，將有 5 + 3 = 8 顆糖果，是孩子中最多的。
- 孩子 4，將有 1 + 3 = 4 顆糖果，不是孩子中最多的。
- 孩子 5，將有 3 + 3 = 6 顆糖果，是孩子中最多的。
```

範例 2：

```coffee
輸入: candies = [4,2,1,1,2], extraCandies = 1
輸出: [true,false,false,false,false]
說明: 只有 1 顆額外的糖果。
即使給了另一個孩子額外的糖果，孩子 1 也總是擁有最多數量的糖果。
```

範例 3：

```coffee
輸入: candies = [12,1,12], extraCandies = 10
輸出: [true,false,true]
```

## 解題

```rs
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // 先計算 candies 陣列中目前的最大糖果數
        let max_candies = *candies.iter().max().unwrap();

        // 對每個小孩，檢查 candies[i] + extraCandies 是否大於或等於最大值。
        candies
            .into_iter()
            .map(|candy| candy + extra_candies >= max_candies)
            .collect()
    }
}
```
