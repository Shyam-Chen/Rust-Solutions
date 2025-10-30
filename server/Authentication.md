# 身分驗證 (Authentication)

```sh
$ cargo add jsonwebtoken
```

## 密碼雜湊 (Password Hashing)

```sh
$ cargo add argon2
```

### 初始化密碼

用於忘記密碼執行重設密碼或新帳戶開通時預設密碼。

```rs
use rand::distr::{Alphanumeric, SampleString};

fn main() {
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    println!("Random password: {password}");
}
```

## 多重要素驗證 (Multi-factor Authentication)

## 帳戶授權 (Authorization)

### Sign in with Google

### Sign in with Microsoft

### Sign in with Apple
