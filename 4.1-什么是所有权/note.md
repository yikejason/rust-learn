### 什么是所有权

- 所有权是 Rust 最为与众不同的特性，它让 Rust 无需垃圾回收即可保证内存安全

## 所有权规则

- Rust 所有权规则如下：

1. Rust 中的每个值都有一个被称为所有者的变量，即值的所有者是某个变量。
2. 值在任何时刻有且仅有一个所有者
3. 当所有者离开作用域后，这个值将被丢弃

fn main() {
let a: u32 = 8;
let b: String = String::from("hello");
let c: Vec<u8> = vec![1, 2, 3];
}

上面代码中 a 是 8 的所有者，b 是 String::from("hello")的所有者，c 是 vec![1, 2, 3] 的所有者。
注意 b 是 String::from("hello")的所有者，但是 b 不是"hello"的所有者，同理，c 是 vec![1, 2, 3]的所有者，但不是[1, 2, 3]的所有者。

## 变量作用域

- 变量作用域是变量程序中有效的范围。一对花括号表示的范围就是作用域，变量有效范围就是从创建开始，到离开作用域结束。

例子 1
fn f() {
    let b = 1u32;            // ---------------------------------|
    let c = 2u32;            // -----------|                     |
                             //            |                     |
                             //            |                     |--- b的作用域范围
    println!("b = {:?}", b); //            |-- c的作用域范围     |
    println!("c = {:?}", c); //            |                     |
                             // -----------|---------------------|
}

fn main() {
    let a: u32 = 8; // ------------------------------|
    println!("a = {:?}", a);                      // |
                                                  // |---- a的作用域范围
    f();                                          // |
// --------------------------------------------------|
}


例子 2
fn main() {
    let a = 8u32;                 // --------------------------|
    {                             //                           |
        let b = 5u32;             // -------|                  |
        println!("a = {:?}", a);  //        |-- b的作用域范围  |
        println!("b = {:?}", b);  //        |                  |---- a的作用域范围
                                  // -------|                  |
    }                             //                           |
    println!("a = {:?}", a);      //                           |
                                  // --------------------------|
}

## String类型

# String类型创建的三种方式

String::from()  用于将其他类型转换为字符串
to_string()     用于将其他类型转换为字符串
String::new()   创建一个空的字符串对象

fn main() {
  let s1 = String::from("Hello"); // 方法一
  let s2 = "Hello".to_string(); // 方法二
  let mut s3 = String::new();
  s3.push("H");
  s3.push("e");
}
String可变 而 字符串字面量不行

# 内存与分配
- 在Rust中，编译大小确定放在栈上，编译大小不能确定的数据放在堆上。
fn main() {
  let mut s = String::new();
  s.push('A');
  s.push('B');

  println("{s}"); // 打印AB
}

在代码第2行定义String类型时，并不能确定最终字符串的大小，所以字符串内容本身应该存储在堆上

# 变量与数据的交互方式 (一)：移动

- 1. 完全存储在栈上的类型
fn main() {
  let x = 5u32;
  let y = x;
  println!("x: {:?}, y: {:?}", x, y);
}

x 和 y 都是 u32类型，在编译时知道大小，都存储在栈上。代码第2行是将 5 绑定在变量x上，第三行则是通过自动拷贝的方式将 5 绑定到 y 上， 所以当 let y = x 发生后 这段代码最后两个值都是 5 ， 分别绑定到了 x 和 y 上

- 2. 涉及到堆存储的类型
fn main() {
  let s = "Hello world".to_string();
  let s1 = s;
  // println!("s: {:?}", s) // 报错 s 的所有权转移到了 s1  
  println!("s1: {:?}", s1);
}

当let s1 = s执行后，就发生了所有权的转移，String类型值的所有权从s转移到了s1。此时Rust认为原来的s不再有效。因此，上面代码第4行打开编译将会出错。

图解详见 rust 轻松学 https://rustycab.github.io/LearnRustEasy/chapter_3/chapter_3_7_1.html

# 变量与数据的交互方式 (一)：克隆













