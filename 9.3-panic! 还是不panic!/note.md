### 总体原则
- 在定义一个可能失败的函数时，优先考虑返回 Result
- 否则就panic!

## 编写示例、原型代码、测试
- 可以使用panic
  - 演示某些概念：unwrap
  - 原型代码：unwrap、expect
  - 测试：unwrap、expect

## 有时你比编译器掌握更多的信息
- 你可以确定Result就是OK: unwrap
eg: 
use std::net::IpAddr;

fn main() {
  let home: IpAddr = "127.0.0.1".parse().unwrap();
}

## 错误处理的指导性建议
# 场景建议
- 调用你的代码，传入无意义的参数值：panic!
- 调用外部不可控代码，返回非法状态，你无法修复: panic
- 如果失败是可预期的：Result
- 当你的代码对值进行操作，首先应该验证这些值的合法性：panic!

### 为验证创建自定义类型
fn main() {
  loop {
    // ...
    let guess = "32";
    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    }

    if guess < 1 || guess > 100 {
      println!("The score number will be between 1 and 100");
      continue;
    }
    // ...
  }
}

- 创建新的类型，把验证逻辑放在构造实例的函数里
修改上述例子：
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}", value);
    }

    Guess { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }

}


fn main() {
  loop {
    // ...
    let guess = "32";
    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

     let guess = Guess::new(guess);

    }
    // ...
  }
