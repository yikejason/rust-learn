## use 关键字
- 可以使用use关键字将路径导入到作用域内
  - 仍遵循私有性原则
- 使用绝对路径
eg: 
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn some_function() {}
  }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

- 使用use来指定相对路径 eg:
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn some_function() {}
  }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

## use的习惯用法
- 函数：将函数的父级模块引入到作用域 （指定到父级）
- struct enum 其他：指定完整路径 （指定到本身）
eg:
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);
}

## 同名条目指定到父级
eg:
use std::fmt;
use std::io;

fn f1() -> fmt::Result {}

fn f2() -> io::Result {}

fn main() {}

- 第二种做法 as 关键字
  - as关键字可以为引入的路径指定本地的别名
eg:
use std::fmt::Result;
use std::io:Result as ioResult;

fn f1() -> Result {}

fn f2() -> ioResult {}

fn main() {}


## 使用 pub use 重新导出名称
- 使用 use 将路径（名称）导入到作用域后，该名称在此作用域内是私有的。

- pub use：重导出
  - 将条目引入作用域
  - 该条目可以被外部代码引入到他们的作用域

eg:
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

use crate::front_of_house::hosting;  // 用于当前作用域
pub use crate::front_of_house::hosting; // 该条目可以被外部代码引入到他们的作用域

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
}

## 使用外部包
1. Cargo.toml 添加依赖项的包 （package）
   - https://crate.io/
2. use将特定条目引入作用域


## 使用嵌套路径清理大量的use语句
- 如果使用同一个包或模块下的多个条目
- 可使用嵌套路径在同一行内将上述条目进行引入：
  - 路径相同的部分::{路径差异的部分}
eg:
// use std::cmp::ordering
// use std::io
use std::{cmp::Ordering, io}

fn main() {}

- 特殊情况 如果两个use路径之一是另一个的子路径
use std::io;
use std::io::Write;

上面的等价于 use std::io::{self, Write};

## 通配符 *
- 使用*可以把路径中所有的公共条目都引入到作用域中
- 注意：谨慎使用
- 应用场景：
  - 测试 将所有测试代码引入到tests模块
use std::collections::*
