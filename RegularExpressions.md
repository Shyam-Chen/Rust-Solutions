# 正規表達式 (Regular Expressions)

```sh
$ cargo add regex
```

```rs
use regex::Regex;

fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

fn main() {
    let emails = [
        "example@test.com",
        "invalid-email",
        "user.name+tag@test.com",
        "user@sub.domain.com",
    ];

    for email in &emails {
        println!(
            "{email} is valid? {is_valid}",
            is_valid = is_valid_email(email)
        );
    }
}
// example@test.com is valid? true
// invalid-email is valid? false
// user.name+tag@test.com is valid? true
// user@sub.domain.com is valid? true
```
