### 接收命令行参数
use std::env;

fn main() {
  let args = env::args().collect();
  let query = &args[1];
  let filename = &args[2];
}

### 读取文件
use std::env;
use std::fs;

fn main() {
  let args = env::args().collect();
  let query = &args[1];
  let filename = &args[2];

  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  println!("With text:/n{}", contents);
}

### 重构以改进模块化与错误处理

### 采用测试驱动开发完善库的功能

### 处理环境变量

### 将错误信息输出到标准错误而不是标准输出


# mac 中设置环境变量
在 macOS 中，你可以使用 export 命令在终端命令行中设置环境变量。要设置 CASE_INSENSITIVE 环境变量为 1，可以执行以下命令：
export CASE_INSENSITIVE=1


在 macOS 中，你可以使用 export 命令在终端命令行中设置环境变量。要设置 CASE_INSENSITIVE 环境变量为 1，可以执行以下命令：

bash
Copy code
export CASE_INSENSITIVE=1
这将在当前终端会话中设置 CASE_INSENSITIVE 环境变量为 1。请注意，这种设置只在当前会话中有效，如果你关闭终端或启动新的终端会话，这个设置将会丢失。

如果你想要永久设置这个环境变量，可以将上述命令添加到你的 shell 配置文件中，例如 ~/.bash_profile（对于 Bash）或 ~/.zshrc（对于 Zsh）。这样，每次启动终端时，都会自动设置这个环境变量。

