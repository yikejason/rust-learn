# 查看 cargo 版本
cargo --version

# cargo 创建项目
cargo new project

# cargo.toml 结构 配置文件 类似js 中的 package.json
[package]             包
name = "hello_cargo"  项目名称
version = "0.1.0"     项目版本
edition = "2021"      Rust大版本号

[dependencies]       （代码包被称为crate）

# cargo build
- 1. 构建rust项目 
- 2. 运行还需要手动操作一步 ./target/debug/hello_cargo

# cargo run
- 一步构建并运行rust项目

# cargo check
- 快速检查代码确保其可以编译，但并不产生可执行文件  构建项目而无需生成二进制文件来检查错误
- cargo check 比 cargo build 快的多，编写代码时定期运行 cargo check 确保代码能够编译。当准备好使用可执行文件时才运行 cargo build

# 发布构建 cargo build --release
- 优化项目
- 更长的编译时间
- 如果你要对代码运行时间进行基准测试，请确保运行 cargo build --release 并使用 target/release 下的可执行文件进行测试。




