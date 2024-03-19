### Package, Crate, Module
## Rust 的代码组织

代码组织主要包括：
 - 哪些细节可以暴露，哪些细节是私有的
 - 作用域内哪些名称有效
 - ...

模块系统
- Package (包)：Cargo的特性，让你构建、测试、共享crate
- Crate (单元包)：模块树，它可以产生一个library或可执行文件
- Module (模块)：use: 让你控制代码的组织、作用域、私有路径
- Path (路径)：为struct、function或module等项命名的方式

# Package和Crate
Crate 的类型:
- binary (二进制)
- library (库)

Crate Root:
- 是源代码文件
- Rust 编译器从这里开始，组成你的Crate的根Module

一个 Package:
- 包含一个cargo.toml, 它描述了如何构建这些Crates
- 只能包含 0 - 1 个 library(库) crate
- 可以包含任意数量的 binary（二进制）crate
- 但是至少包含一个crate(library 或 binary)

# Cargo 的惯例
- src/main.rs:
  - 它是 binary crate 的 crate root
  - crate的名与package名相同
- src/lib.rs:
  - package包含一个library crate
  - 它是 library crate 的 crate root
  - crate名 与 package 名相同

Cargo 把 create root文件交给 rustc 来构建library 或 binary

# Crate 的作用
- 将相关功能组合到一个作用域内，便于在项目间进行共享
- 防止冲突
eg: rand crate, 访问它的功能需要通过它的名字：rand

