# Beginning - Algorithms

## 陣列 (Array)

訪問元素:

```rs
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog.";

    if let Some(c) = sentence.chars().nth(4) {
        println!("{c}"); // q
    }
}
```

```rs
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog.";
    println!("{}", &sentence[4..=4]); // q
}
```

```rs
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog.";
    println!("{}", sentence.as_bytes()[4] as char); // q
}
```

```rs
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog.";
    let chars: Vec<char> = sentence.chars().collect();
    println!("{}", chars[4]); // q
}
```

初始化陣列:

```rs
fn main() {
    let arr = [0; 10];
    println!("{arr:?}");

    let arr = [0; 10].to_vec();
    println!("{arr:?}");

    let arr = Vec::from([0; 10]);
    println!("{arr:?}");

    let arr = vec![0; 10];
    println!("{arr:?}");

    let arr: [i32; 10] = vec![0; 10].try_into().unwrap();
    println!("{arr:?}");

    let arr: Vec<i32> = (0..10).map(|_| 0).collect();
    println!("{arr:?}");

    let arr: [i32; 10] = std::array::from_fn(|_| 0);
    println!("{arr:?}");
}
// 皆輸出: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

插入元素:

```rs
fn main() {
    let mut arr = vec!["A", "B", "C", "D", "E"];
    arr.insert(3, "Z"); // 在索引 3 處插入 Z
    println!("{arr:?}");

    let mut arr = vec!["A", "B", "C", "D", "E"];
    arr.splice(3..3, vec!["Z"]); // 在索引 3 處插入 Z，可插入多個元素
    println!("{arr:?}");
}
// 皆輸出: ["A", "B", "C", "Z", "D", "E"]
```

```rs
fn main() {
    let mut arr = vec!["A", "B", "C", "D", "E"];

    // 找到 C 的索引位置
    if let Some(index) = arr.iter().position(|&x| x == "C") {
        // 插入 Z 到 C 之後的位置
        arr.insert(index + 1, "Z");
    }

    println!("{arr:?}"); // ["A", "B", "C", "Z", "D", "E"]
}
```

刪除元素:

```rs
fn main() {
    let mut arr = vec!["A", "B", "C", "D", "E"];

    if let Some(index) = arr.iter().position(|&x| x == "C") {
        arr.remove(index); // 根據索引移除元素，只會移除第一個出現的
    }

    println!("{arr:?}"); // ["A", "B", "D", "E"]
}
```

```rs
fn main() {
    let mut arr = vec!["A", "B", "C", "D", "E"];
    arr.retain(|&x| x != "C"); // 保留陣列中符合條件的元素，也就是刪除不符合條件的元素。
    println!("{arr:?}"); // ["A", "B", "D", "E"]
}
```

```rs
fn main() {
    let arr = vec!["A", "B", "C", "D", "E"];
    // 不修改原始陣列，而是建一個新陣列
    let filtered: Vec<_> = arr.into_iter().filter(|&x| x != "C").collect();
    println!("{filtered:?}", ); // ["A", "B", "D", "E"]
}
```

### 雙指標 (Two Pointers)

相關名詞:

- 弗洛伊德判圈演算法 (Floyd’s Cycle Detection Algorithm)
- 龜兔賽跑演算法 (Tortoise and Hare Algorithm)
- 快慢指標 (Fast and Slow Pointers)

移動數字 0 至末尾:

```rs

```

## 鏈結串列 (Linked List)

### 單向鏈結串列 (Singly Linked List)

Rust 不允許「遞迴型別」沒有間接層 (indirection)，因為編譯器不知道 `ListNode` 的大小是多少 (它裡面又包含另一個 `ListNode`，無限展開)，所以需使用 `Box<ListNode>`:

- `Box<T>` 是一個固定大小的「指標」
- 指向堆積 (Heap) 上真實儲存的 `T`

```rs
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        ListNode { value, next: None }
    }
}
```

```rs
fn main() {
    let mut head = None;

    for value in ["E", "D", "C", "B", "A"] {
        head = Some(Box::new(ListNode { value, next: head }));
    }

    let mut current = head.as_ref();

    while let Some(node) = current {
        print!("{}", node.value);

        if node.next.is_some() {
            print!(" -> ");
        }

        current = node.next.as_ref();
    }

    println!();
}
// A -> B -> C -> D -> E
```

#### 雙指標 (Two Pointers)

找出鏈結串列的中間節點:

- 慢指標每次移動一步
- 快指標每次移動兩步

```rs

```

### 雙向鏈結串列 (Doubly Linked List)

```rs
use std::collections::LinkedList;

fn main() {
    // 建立一個空的 LinkedList
    let mut list: LinkedList<String> = LinkedList::new();

    // 插入元素
    list.push_back("A".into());
    list.push_back("B".into());
    list.push_back("C".into());
    list.push_front("D".into());

    // 打印整個 LinkedList
    println!("List: {list:?}"); // List: ["D", "A", "B", "C"]

    // 查看頭尾元素
    println!("Front: {:?}", list.front()); // Front: Some("D")
    println!("Back: {:?}", list.back()); // Back: Some("C")

    // 移除元素
    list.pop_front();
    list.pop_back();

    // 打印移除元素後的整個 LinkedList
    println!("After pop: {list:?}"); // After pop: ["A", "B"]

    let mut list2: LinkedList<String> = LinkedList::new();
    list2.push_back("E".into());
    list2.push_back("F".into());

    list.append(&mut list2);

    // 打印附加 list2 後的整個 LinkedList
    println!("After append: {list:?}"); // After append: ["A", "B", "E", "F"]
}
```

## 堆疊 (Stack)

以後進先出 (Last In, First Out，LIFO) 為原則。

```rs
#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // 將元素加入堆疊
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // 移除並返回堆疊頂部的元素
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // 查看堆疊頂部的元素
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // 檢查堆疊是否為空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // 返回堆疊中的元素數量
    pub fn size(&self) -> usize {
        self.items.len()
    }

    // 清空堆疊
    pub fn clear(&mut self) {
        self.items.clear();
    }
}
```

```rs
fn main() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("{:?}", stack.peek()); // Some(30)
    println!("{:?}", stack.pop()); // Some(30)
    println!("{:?}", stack.size()); // 2
    println!("{:?}", stack.is_empty()); // false
    stack.clear();
    println!("{:?}", stack.is_empty()); // true
}
```

基於鏈結串列 (Linked List) 實現:

```rs

```

## 佇列 (Queue)

以先進先出 (First In First Out，FIFO) 為原則。

```rs
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    // 入列
    queue.push_back("A");
    queue.push_back("B");
    queue.push_back("C");

    // 處理最先入列的資料
    while let Some(front) = queue.pop_front() {
        println!("Processing: {front}");
    }
}
// Processing: A
// Processing: B
// Processing: C
```

### 滑動視窗 (Sliding Window)

```rs
use std::collections::VecDeque;

fn main() {
    // 宣告容量為 3 的 VecDeque 作為滑動視窗
    let mut sliding_window = VecDeque::with_capacity(3);

    // 模擬資料流的輸入
    let stream_data = vec![1, 2, 3, 4, 5];

    for data in stream_data {
        // 保持視窗的大小，超過容量時移除最舊的數據
        if sliding_window.len() == sliding_window.capacity() {
            sliding_window.pop_front();
        }

        // 新資料推入視窗
        sliding_window.push_back(data);

        // 此時可以使用滑動視窗進行一些操作
        println!("當前滑動視窗: {sliding_window:?}");
    }
}
// 當前滑動視窗: [1]
// 當前滑動視窗: [1, 2]
// 當前滑動視窗: [1, 2, 3]
// 當前滑動視窗: [2, 3, 4]
// 當前滑動視窗: [3, 4, 5]
```

加上操作:

```rs
use std::collections::VecDeque;

fn main() {
    // 宣告容量為 3 的 VecDeque 作為滑動視窗
    let mut sliding_window = VecDeque::with_capacity(3);

    // 模擬資料流的輸入
    let stream_data = vec![1, 2, 3, 4, 5];

    // 宣告用來追蹤滑動視窗元素總和的變數
    let mut window_sum = 0;

    for data in stream_data {
        // 保持視窗的大小，超過容量時移除最舊的數據
        if sliding_window.len() == sliding_window.capacity() {
            if let Some(removed) = sliding_window.pop_front() {
                window_sum -= removed; // 從總和中扣掉被移除的數字
            }
        }

        // 新資料推入視窗
        sliding_window.push_back(data);

        // 此時可以使用滑動視窗進行一些操作
        window_sum += data; // 將新資料加到總和裡
        println!("當前滑動視窗: {sliding_window:?}, 總和: {window_sum}");
    }
}
// 當前滑動視窗: [1], 總和: 1
// 當前滑動視窗: [1, 2], 總和: 3
// 當前滑動視窗: [1, 2, 3], 總和: 6
// 當前滑動視窗: [2, 3, 4], 總和: 9
// 當前滑動視窗: [3, 4, 5], 總和: 12
```

## 雜湊表 (Hash Table)

```rs
use std::collections::HashMap;
```

```rs
use std::collections::HashSet;
```

```rs
use hashbrown::{HashMap, HashSet};
```

## 二元樹 (Binary Tree)

```rs
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}
```

## 二元搜尋樹 (Binary Search Tree)

## 平衡樹 (AVL / Red-Black Tree)

## 堆積 (Heap)

## 字典樹 (Trie)

## 圖 (Graph)

## 排序 (Sorting)

### 泡沫排序 (Bubble Sort)

依次比較相鄰的兩個元素，並將較大的元素交換到後面。

### 選擇排序 (Selection Sort)

每次從未排序的部分中選出最小的元素，並將其放到已排序部分的最後。

### 堆積排序 (Heap Sort)

### 合併排序 (Merge Sort)

基於分治 (Divide and Conquer) 策略。

## 搜尋 (Searching)

## 分治 (Divide and Conquer)

## 回溯 (Backtracking)

## 動態規劃 (Dynamic Programming)

## 貪婪 (Greedy)

## 位元操作 (Bit Manipulation)
