# 控制流程 (Control Flow)

## 條件判斷 (If/Else)

```rs
fn main() {
    let score = 85;

    if score >= 60 {
        println!("Pass");
    } else {
        println!("Fail");
    }
}
// Pass
```

```rs
fn main() {
    let number = -7;

    if number > 0 {
        println!("Positive");
    } else if number == 0 {
        println!("Zero");
    } else {
        println!("Negative");
    }
}
// Negative
```

```rs
fn main() {
    let age = 20;
    let can_vote = if age >= 18 { "Yes" } else { "No" };
    println!("Can vote? {can_vote}");
    // Can vote? Yes
}
```

## 模式比對 (Match)

```rs
fn main() {
    let number = 2;

    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"), // _ 是萬用匹配符
    }
}
// 二
```

```rs
fn main() {
    let number = 7;

    match number {
        n if n % 2 == 0 => println!("偶數: {n}"),
        n if n % 2 == 1 => println!("奇數: {n}"),
        _ => println!("未知"),
    }
}
// 奇數: 7
```

## 無限迴圈 (Loops)

```rs
fn main() {
    let mut count = 0;

    loop {
        count += 1;

        if count == 2 {
            continue; // 跳過當前計數為 2
        }

        println!("Count = {count}",); // 當前計數

        if count == 3 {
            break; // 跳出 loop
        }
    }

    println!("Loop 結束");
}
// Count = 1
// Count = 3
// Loop 結束
```

回傳值:

```rs
fn main() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // 回傳值
        }
    };

    println!("結果是: {result}");
}
// 結果是: 20
```

## 條件式迴圈 (While Loops)

```rs
fn main() {
    let mut count = 3;

    while count != 0 {
        println!("{count}");
        count -= 1;
    }

    println!("結束");
}
// 3
// 2
// 1
// 結束
```

## 疊代器迴圈 (For Loops)

```rs
fn main() {
    for number in 1..4 {
        println!("{number}");
    }

    println!("結束");
}
// 1
// 2
// 3
// 結束
```

包含:

```rs
fn main() {
    for number in 1..=3 {
        println!("{number}");
    }

    println!("結束");
}
// 1
// 2
// 3
// 結束
```
