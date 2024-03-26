use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 接收命令行参数
    let args: Vec<String> = env::args().collect();

    // 解析参数
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // 读取文件
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
