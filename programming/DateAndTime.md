# 日期和時間 (Date and Time)

```sh
$ cargo add chrono
```

取得當前時間:

```rs
use chrono::prelude::*;

fn main() {
    let utc = Utc::now();
    println!("{utc}");
}
```

建立特定時間:

```rs
use chrono::prelude::*;

fn main() {
    let date_time_1 = Utc.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap();

    let date = NaiveDate::from_ymd_opt(2024, 7, 7).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let date_time_2 = NaiveDateTime::new(date, time).and_utc();
    println!("{}", date_time_1 == date_time_2);
    // true

    let date_time_3 = NaiveDate::from_ymd_opt(2024, 7, 7)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    println!("{}", date_time_1 == date_time_3);
    // true

    let s = "2024-7-7 0:0:0"; // "2024-07-07 00:00:00"
    let date_time_4 = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .and_utc();
    println!("{}", date_time_1 == date_time_4);
    // true
}
```
