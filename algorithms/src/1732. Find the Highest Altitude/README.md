# 1732. 找到最高海拔 (Find the Highest Altitude)

有一個騎士正在進行公路旅行。公路旅行由 `n + 1` 個不同海拔的點組成。騎士從海拔為 `0` 的點 `0` 開始行程。

給定一個長度為 `n` 的整數陣列 `gain`，其中 `gain[i]` 是全部點 `i`​​​​​​ 和 `i + 1` 之間的**淨海拔高度差** (`0 <= i < n`)。回傳**最高海拔**的點。

範例 1：

```coffee
輸入: gain = [-5,1,5,0,-7]
輸出: 1
說明: 海拔高度依序為 [0,-5,-4,1,1,-6]。最高海拔為 1。
```

範例 2：

```coffee
輸入: gain = [-4,-3,-2,-1,4,3,2]
輸出: 0
說明: 海拔高度依序為 [0,-4,-7,-9,-10,-6,-3,-1]。最高海拔為 0。
```

## 解題

```rs
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        // 前綴和陣列
        let mut prefix = Vec::with_capacity(gain.len() + 1);
        prefix.push(0);

        for &g in &gain {
            let last = *prefix.last().unwrap();
            prefix.push(last + g);
        }

        // 找出前綴和陣列中的最大值
        *prefix.iter().max().unwrap()
    }
}
```
