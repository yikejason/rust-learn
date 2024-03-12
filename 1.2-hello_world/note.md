# rust 更新

rustup update

# rust 卸载

rustup self uninstall

# 检查 rust 版本

rustc --version

# 编译文件

rustc main.rs
编译成功后，Rust 就会输出一个二进制可执行文件

编译成功后 mac 和 linux 会生成两个文件
$ ls
main main.rs

编译成功后 windows 会生成三个文件

> dir /B %= the /B option says to only show the file names =%
> main.exe
> main.pdb
> main.rs
