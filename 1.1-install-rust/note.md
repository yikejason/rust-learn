# 安装
# 在linux和macOS上安装
- 如果是在linux和macOS上，打开终端并输入以下命令
- curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
- 这个命令将会下载安装一个 rustup 工具
- 安装c环境 xcode-select --install

# 在windows上安装
- Windows 上安装 Rust 需要有 C++ 环境，以下为安装的两种方式：
# 第一种
- 1. x86_64-pc-windows-msvc（官方推荐）
先安装 Microsoft C++ Build Tools，勾选安装 C++ 环境即可。安装时可自行修改缓存路径与安装路径，避免占用过多 C 盘空间。安装完成后，Rust 所需的 msvc 命令行程序需要手动添加到环境变量中，否则安装 Rust 时 rustup-init 会提示未安装 Microsoft C++ Build Tools，其位于：%Visual Studio 安装位置%\VC\Tools\MSVC\%version%\bin\Hostx64\x64（请自行替换其中的 %Visual Studio 安装位置%、%version% 字段）下。

如果你不想这么做，可以选择安装 Microsoft C++ Build Tools 新增的“定制”终端 Developer Command Prompt for %Visual Studio version% 或 Developer PowerShell for %Visual Studio version%，在其中运行 rustup-init.exe。

准备好 C++ 环境后开始安装 Rust：

在 RUSTUP-INIT 下载系统相对应的 Rust 安装程序，一路默认即可。

PS C:\Users\Hehongyuan> rustup-init.exe
......
Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation

# 第二种
相比于 MSVC 版本来说，GNU 版本具有更轻量，更靠近 Linux 的优势。

首先，根据 MSYS2 官网 配置 MSYS。

若您觉得下载太慢，可以试试由 Caviar-X 提供的 代理。

在安装 mingw-toolchain 后，请将 %MSYS 安装路径%\mingw64\bin 添加到系统变量 PATH 中。

配置好后，在 MSYS 中输入下面的命令来安装 rustup。

$ curl https://sh.rustup.rs -sSf | sh
之后，根据以下输出进行配置。

Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>2

I'm going to ask you the value of each of these installation options.
You may simply press the Enter key to leave unchanged.

Default host triple? [x86_64-pc-windows-msvc]
x86_64-pc-windows-gnu

Default toolchain? (stable/beta/nightly/none) [stable]
stable

Profile (which tools and data to install)? (minimal/default/complete) [default]
complete

Modify PATH variable? (Y/n)
Y

Current installation options:

   default host triple: x86_64-pc-windows-gnu
     default toolchain: stable
               profile: complete
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>
再之后，按下 1，等待。完成后，您就已经安装了 Rust 和 rustup。




