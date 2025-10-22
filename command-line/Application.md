# 應用程式 (Application)

```sh
$ cargo new my-cli
$ cd my-cli
```

```sh
$ cargo add clap -F derive
```

```rs
use clap::Parser;

#[derive(Parser)]
#[command(name = "my-cli-app")]
#[command(about = "我的第一個 CLI 工具", version)]
struct Cli {
    #[arg(short, long, default_value = "World", help = "輸入你的名字")]
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
}
```

```sh
$ cargo run
Hello, World!

$ cargo run -- --name Alice
Hello, Alice!

$ cargo run -- -n Bob
Hello, Bob!
```

```sh
$ cargo run -- -h
我的第一個 CLI 工具

Usage: my-cli [OPTIONS]

Options:
  -n, --name <NAME>  輸入你的名字 [default: World]
  -h, --help         Print help
  -V, --version      Print version
```

```sh
$ cargo run -- -V
my-cli-app 0.1.0
```

---

```sh
$ cargo add tokio -F full
```
