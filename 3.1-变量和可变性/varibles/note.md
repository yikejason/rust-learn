# 变量和可变性
# 变量
- 默认情况下变量是不可变的，在Rust中，将值和变量关联的过程称为绑定，变量的绑定可以使用let关键字
eg:
  fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
  }

使用 cargo run 运行 会报一个变量不可二次赋值的错

# 可变性

- 我们可以在变量名前加上 mut 使得他们可变，增加 mut 的操作还向以后读代码的人传达了代码的其他部分将会改变这个变量值。
eg:
fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}

使用 cargo run 可以正常运行

# 常量

- 与不可变变量类似 用 const 声明
- 1. 常量不允许使用 mut
- 2. 常量可以在任意作用域，包括全局作用域 这对代码中很多部分知道一个值特别有用
- 3. 常量只能设置为表达式，而不能设置为函数调用的结果或是只能在运行时计算得到的值
     eg: const THREE_HOURS_IN_SECONDS: u32 = 60 _ 60 _ 3;
     Rust 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词

# 遮蔽

- 1.你可以声明和前面变量具有相同名称的新变量，第一个变量可以被第二个声明的变量覆盖
- 2.遮蔽和将变量标记为 mut 的方式不同， 再次使用 let 关键字时有效创建了一个新的变量

fn main() {
let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

}

- mut 必须赋值相同的变量类型 期待是 string 返回的却是 u32 类型 报错
  下面的类型报错
  fn main() {
    let mut spaces = " ";
    spaces = spaces.len();
  }
  正确
  fn main() {
    let mut spaces = " ";
    let spaces = spaces.len();
  }
