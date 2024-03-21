## 字符串是什么
- Rust 的核心语言层面，只有一个字符串类型：字符串切片 str (或&str)
- 字符串切片：对存储在其它地方、UTF-8编码的字符串的引用
  - 字符串字面值：存储在二进制文件中，也是字符串切片


- String 类型
- 来自 标准库 而不是核心语言
- 可增长、可修改、可拥有所有权

## 通常说的字符串是指
- String 和 &str
  - 标准库里用的多
  - UTF-8编码

## 创建一个新的字符串 （String）
- 很多 Vec<T>的操作都可以用于String
- String::new() 函数

fn main() {
  let mut s = String::new();
}

- 使用初始值来创建String:
 - to_string()方法，可用于实现Display trait的类型，包括字符串字面值

fn main() {
  let data = "initial contents";
  let s = data.to_string();
  let s1 = "initial contents".to_string();
}

 - String::from()函数，从字面值创建String
 fn main() {
  let s = String::from("initial contents");
 }

 ## 更新String
 - push_str() 方法：把一个字符串切片附加到String
 
 fn main() {
  let mut s = String::from("foo");
  s.push_str("bar");
  println!("{}", s);
 }

 - push() 方法：把单个字符附加到String 

 fn main() {
  let mut s = String::from("lo");
  s.push("l");
 }

 - +: 连接字符串
   - 使用了类似这个签名的方法 fn add(self, s: &str) -> String {...}
     - 标准库中的add方法使用了泛型
     - 只能把 &str 添加到 String
     - 解引用强制转换 （deref coercion）
 fn main() {
  let s1 = String::from("hello, ");
  let s2 = String::from("World!");

  let s3 = s1 + &s2

  println!("{}", {});
 }

 - format!: 连接多个字符串 不会取得参数的所有权
fn main() {
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  // let s3 = s1 + "-" + &s2 + "-" + &s3;
  // println!("{}", s3);

  let s = format!("{}-{}-{}", s1, s2, s3);

  println!("{}", s);
}

## 对 String 按索引的形式进行访问
- 按索引语法访问 String 的某部分，会报错

## String内部表示
- String是对 Vec<u8> 的包装
  - len() 方法
fn main() {
  let len = String::from("Hola").len();
  println!("{}", len);
}

## 字节 标量值 字形簇
- Rust 有三种看待字符串的方式
- 字节
fn main() {
  let w = "नमस्ते"; // 梵文书写的印度语单词
  for b in w.bytes() {
    println!("{}", b);
  }
}
- 标量值
fn main() {
  let w = "नमस्ते"; // 梵文书写的印度语单词
  for b in w.chars() {
    println!("{}", b);
  }
}

- 字形簇 最接近所谓的字母


- Rust不允许对String进行索引的最后一个原因
  - 索引操作应消耗一个常量时间(o(1))
  - 而String无法保证：需要遍历所有内容，来确定有多少个合法的字符

## 切割字符串
 - 可以使用 [] 和一个范围来创建字符串的切片
 - 必须谨慎使用
 - 如果切割时跨越了字符边界，程序就会panic
fn main() {
  let hello = "Здравствуйте";
  let s = &hello[0..4];
  println!("{}", s);
}

## 遍历String的方法
- 对于标量值： chars() 方法
- 对于字节：bytes() 方法
- 对于字形簇： 很复杂，标准库未提供  可以找下第三方库










