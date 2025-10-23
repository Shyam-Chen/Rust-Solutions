# 智慧指標 (Smart Pointers)

## 堆積 (Heap) `Box<T>`

- 擁有唯一所有權
- 離開作用域時自動釋放記憶體

## 參考計數 (Reference Counted) `Rc<T>`

多重所有權 + 共享相同資料

並行 (Concurrency) 使用改 `Arc<T>`。

## 可變借用檢查 `RefCell<T>`

並行 (Concurrency) 使用改 `Mutex<T>`。
