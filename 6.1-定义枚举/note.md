###
枚举允许我们列举所有的值来定义一个类型

## 定义枚举
IP地址：Ipv4、 Ipv6
enum IpAddKind {
  v4,      // 枚举成员叫变体
  v6,
}

## 枚举值
let five = IpAddKind::v4;
let six = IpAddKind::v6;

five 和 six  都是枚举值   IpAddKind::v4 和 IpAddKind::v6 也是枚举值


## 将数据附加到枚举的变体中
enum IpAddr {
  v4(String),
  v6(String),
}

优点 
- 不使用额外的struct
- 每个变体可以拥有不同的类型以及关联的数据量
enum IpAddr {
  v4(u8, u8, u8, u8),
  v6(String),
}

例子
enum IpAddr {
  v4(u8, u8, u8, u8),
  v6(String),
}

fn main() {
  let home = IpAddr::v4(127, 0, 0, 1);
  let loopback = IpAddr::v6(String::from("::1"));
}

## option枚举
- 定义于标准库中
- 在prelude（预导入模块）中
- 描述了：某个值可能存在（某种类型）或不存在的情况

- Rust中没有null
- Rust中类似Null概念的枚举 - Option<T>
enum Option<T> {
  Some(T),
  None,
}

它包含在Prelude（预导入模块）中。可直接使用：
- Option<T>
- Some(T)
- None

eg:

fn main() {
  let some_number = Some(5);
  let some_string = Some("A String");

  let absent_number: Option<i32> = None;
}

- Option<T> 比 Null 好在哪？
Option<T> 和 T 是不同的类型，不可以把 Option<T> 直接当成T

错误例子示范
fn main() {
  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  let sum = x + y;  // x 和 y不能相加 因为不是相同类型
}

- 若想使用Option<T>中的T, 必须将它转换为T








