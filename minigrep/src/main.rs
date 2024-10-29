// main.rs
use std::env;
use std::process;
use minigrep; // 假设你的库名为 minigrep

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // 解析命令行参数并构造 Config
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 运行搜索功能
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
