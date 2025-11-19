# 238. 除自身之外陣列的乘積 (Product of Array Except Self)

給定一個整數陣列 `nums`，回傳一個陣列 `answer`，使得 `answer[i]` 等於 `nums` 中除 `nums[i]` 之外的所有元素的乘積。

確保 `nums` 之中任何前後綴的乘積都在 **32 位元**整數範圍內。

你必須寫一個在 `O(n)` 時間複雜度內完成解題，且不使用除法運算。

範例 1：

```coffee
輸入: nums = [1,2,3,4]
輸出: [24,12,8,6]
```

範例 2：

```coffee
輸入: nums = [-1,1,0,-3,3]
輸出: [0,0,9,0,0]
```

## 解題

核心思路，觀察 `answer[i]` 是除了自己以外的所有元素乘積：

```coffee
nums = [1, 2, 3, 4]
以 i = 1 來看，除了 nums[1] → 2 以外的整數為 1、3 和 4
left_product = nums[0] = 1
right_product = nums[2] * nums[3] = 3 * 4
answer[i] → answer[1] = left_product * right_product = 1 * 12 = 12
```

```coffee
1. 左乘積陣列：對於每個位置 i，計算它左邊所有元素的乘積。
 - nums[0] * nums[1] * ... * nums[i-1]
2. 右乘積陣列：對於每個位置 i，計算它右邊所有元素的乘積。
 - nums[i+1] * nums[i+2] * ... * nums[n-1]
3. 最終結果：answer[i] = 左乘積 * 右乘積。
```

但為了節省空間複雜度，不建立兩個額外陣列，而是：

```coffee
- 用 answer 先存左乘積。
- 再從右往左走訪，乘上右乘積。
```

```rs
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1; n];

        // 累積左邊的乘積
        let mut left_product = 1;

        for i in 0..n {
            answer[i] = left_product; // 將目前的左乘積存入 answer[i]
            left_product *= nums[i]; // 更新左乘積，乘上當前元素
        }

        // 累積右邊的乘積
        let mut right_product = 1;

        for i in (0..n).rev() {
            answer[i] *= right_product; // 將右乘積乘到 answer[i]，此時 answer[i] 已經有左乘積
            right_product *= nums[i]; // 更新右乘積，乘上當前元素
        }

        answer
    }
}
```

演示 `nums = [1, 2, 3, 4]`：

- 第一個迴圈 (左乘積)：

```coffee
- 初始: answer = [1, 1, 1, 1], left_product = 1
- i = 0: answer[0] = 1, left_product = 1 * 1 = 1
- i = 1: answer[1] = 1, left_product = 1 * 2 = 2
- i = 2: answer[2] = 2, left_product = 2 * 3 = 6
- i = 3: answer[3] = 6, left_product = 6 * 4 = 24
- 結果: answer = [1, 1, 2, 6]
```

- 第二個迴圈 (右乘積)：

```coffee
- 初始: answer = [1, 1, 2, 6], right_product = 1
- i = 3: answer[3] → 6 *= 1 → 6, right_product = 1 * 4 = 4
- i = 2: answer[2] → 2 *= 4 → 8, right_product = 4 * 3 = 12
- i = 1: answer[1] → 1 *= 12 → 12, right_product = 12 * 2 = 24
- i = 0: answer[0] → 1 *= 24 → 24, right_product = 24 * 1 = 24
- 結果: answer = [24, 12, 8, 6]
```
