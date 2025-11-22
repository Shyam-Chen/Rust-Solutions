# 643. 子陣列最大平均 I (Maximum Average Subarray I)

給定一個由 `n` 個元素組成的整數陣列 `nums` 和一個整數 `k`。

找到一個**長度等於** `k` 且平均值最大的連續子陣列並回傳該值。任何計算誤差小於 <code>10<sup>-5</sup></code> 的答案都將被接受。

範例 1：

```coffee
輸入: nums = [1,12,-5,-6,50,3], k = 4
輸出: 12.75000
說明: 最大平均值為 (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
```

範例 2：

```coffee
輸入: nums = [5], k = 1
輸出: 5.00000
```

## 解題

使用滑動視窗

```coffee
 1, 12, -5, -6, 50, 3
{    k = 4    } → 1 + 12 + −5 + −6 → sum = 2

 1, 12, -5, -6, 50, 3
  {     k = 4     } → 12 + −5 + −6 + 50 → sum = 51 (最大)

 1, 12, -5, -6, 50, 3
       {    k = 4    } → −5 + −6 + 50 + 3 → sum = 42

最大平均值 = 最大總和 / k → 51 / 4 = 12.75
```

```rs
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;

        // 計算前 k 個元素的總和，做為初始視窗
        let mut window_sum: i32 = nums[..k].iter().sum();

        // 目前為止看到的最大視窗總和
        let mut max_sum = window_sum;

        // 開始滑動視窗 (從索引 k 開始)
        for i in k..nums.len() {
            // 滑出左邊元素，滑入右邊元素
            window_sum += nums[i] - nums[i - k];

            // 更新最大視窗總和
            if window_sum > max_sum {
                max_sum = window_sum;
            }
        }

        // 回傳最大平均值
        max_sum as f64 / k as f64
    }
}
```
