### 控制流运算符 match
- 允许一个值与一系列模式进行匹配，并执行匹配模式对应的代码
- 模式可以是字面值、变量名、通配符...
eg:
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Penny");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn main() {}

## 绑定值模式匹配
- 匹配的分支可以绑定到被匹配对象的部分值，因此可以中enum变体中提取值
eg:
#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}


enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Penny");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    },
  }
}

fn main() {
  let c = Coin::Quarter(UsState::Alaska);
  println!("{}", value_in_cents(c));
}

## 匹配option<T>
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i) => Some(i + 1),
    None => None,
  }
}

## match匹配必须穷举所有的可能
- _ 通配符：替代其余没列出的值

fn main() {
  let v = 0u8;
  match v {
    1 => println!("one"),
    2 => println!("two"),
    _ => (),
  }
}