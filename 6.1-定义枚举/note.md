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





