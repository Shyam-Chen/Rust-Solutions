# Control Flow

## If/Else

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

## Match

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

## Loops

## While Loops

## For Loops
