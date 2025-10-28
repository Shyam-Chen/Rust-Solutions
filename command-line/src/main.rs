use clap::{Parser, Subcommand};
use indicatif::ProgressBar;
use tokio::time::{Duration, sleep};

#[derive(Parser)]
#[command(name = "my-cli-app")]
#[command(about = "我的第一個 CLI 工具", version)]
struct Cli {
    #[arg(short, long, default_value = "World", help = "輸入你的名字")]
    name: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "安裝指定的 package")]
    Install { package: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);

    if let Some(Commands::Install { package }) = cli.command {
        handle_install(package).await;
    }
}

async fn handle_install(package: String) {
    // 建立旋轉器
    let spinner = ProgressBar::new_spinner();

    // 設定旋轉器的開始訊息
    spinner.set_message(format!("正在準備安裝套件: {}", package));

    // 開始旋轉
    spinner.enable_steady_tick(Duration::from_millis(200));

    // 模擬非同步的安裝作業
    sleep(Duration::from_secs(2)).await;

    // 停止旋轉，並顯示任務完成訊息
    spinner.finish_with_message(format!("套件 {} 安裝完成!", package));
}
