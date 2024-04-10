### 引用与借用

## 引用
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);   

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

这些 & 符号就是引用，它们允许你使用值但不获得所有权

例如上面的例子  &s1是s1的引用


## 借用
获取变量的引用被称为借用   
例子：
let s2 = &s1;   通过获取s1变量的引用，这个行为是借用

calculate_length(&s1) 这个是通过函数参数获取s1变量的引用， 这个行为也是借用

借用和引用其实是一样的

## note 
其实，在 Rust 中，“借用”和“引用”是一个概念，只不过在其他语言中引用的意义和 Rust 不同，所以 Rust 提出了新概念“借用”，便于区分。

## 不可变引用

# 使用可变引用
- 通过可变引用改变变量的值，对一个变量加上 &mut 就是对其的可变引用

fn main() {
  let mut s = String::from("hello");
  change(&mut s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world"); // 可变引用，可以对变量进行修改
}

# 引用的作用域
- 变量的作用域是从定义开始到花括号结束的位置
{
    ...
    let a = 1u32;   // a的作用域开始位置
    ...
} // 花括号之前为a的作用域结束位置

- 引用的作用域和变量的作用域有些区别，在老版本编译器（Rust 1.31 之前）中，引用的作用域和变量的作用域一样，也是从定义的位置开始到花括号之前结束

{
    ...
    let s = "Hello".to_string();
    let r = &s;       // r的作用域开始位置
    ....
}// 花括号之前为r的作用域结束位置


- 引用作用域结束位置从花括号变成最后一次使用的位置
{
  ...
  let s = "hello".String::from('hello');
  let r = &s;                 // r的作用域开始
  println!("r = {:?}", r);    // r的作用域结束
  ... // 后面不再使用 r
}

# 使用可变引用的限制
- 限制一：同一作用域，特定数据只能有一个可变引用
报错例子：
fn main() {
  let mut s1 = String::from("hello");
  let r1 = &mut s1;                 // 
  let r2 = &mut s2;                 //
  println("{}, {}", s1, s2);
}

但是下面的代码可以的(新老编译器都可以)：
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push('!');
        println!("r1: {:?}", r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    r2.push('!');
    println!("r2: {:?}", r2);
}

下面的代码在新编译器中也是可以的：
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push('!');
    println!("r1: {:?}", r1); // 后面的代码不再使用r1, 新编译中r1作用域在此处结束，
                              // 所以完全可以在后面创建一个新的引用
    let r2 = &mut s;
    r2.push('!');
    println!("r2: {:?}", r2);  //老编译器中，r1的作用域在花括号前结束，所以老编译器中此代码编译不过
}



- 限制二：同一作用域不可变引用和可变引用不能同时存在
fn main() {
  let mut s = String::from("hello");
  let r1 = &s; // 没问题
  let r2 = &mut s; // 大问题，同时存在两个s的引用和一个可变引用
  println!("{}", r1);
  println!("{}", r2);
}

新编译器中可编译
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用，  新编译器中： r1和r2离开了其作用域

    let r3 = &mut s; // 没问题，因为r1和r2已不存在，没有同时存在对s的引用和可变引用
    println!("{}", r3);
}       // 老编译器中： r1、r2、r3的作用域都是在花括号之前结束


Rust这样设计的原因是避免数据竞争

## 悬垂引用
-悬垂引用也叫悬垂指针，意思是指针指向某个值后，这个值被释放掉了，而指针依然存在，其指向的内存不存在任何值或者已被其他变量重新使用
eg: 下面代码形成了悬垂指针
fn main() {
  let reference_no_thing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");
  &s   // s 在离开花括号前离开作用域，将会变得无效，  返回指向s的引用将会是个悬垂引用
}

正确的代码
fn main() {
  let reference = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");
  s      //s 虽然离开了函数作用域的范围，但是所有权被转移出去了，值没有被释放
}

#### 引用的总结
1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
2. 引用必须有效，（不能是悬垂引用）


