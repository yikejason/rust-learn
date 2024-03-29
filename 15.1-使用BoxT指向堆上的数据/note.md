- Box<T> 是最简单的智能指针
  - 允许你在 heap 上存储数据 （而不是stack）
  - stack 上是指向 heap 数据的指针
  - 没有性能开销
  - 没有其他额外的功能

- 实现了 Deref trait 和 Drop trait


## Box<T> 的常用场景
- 在编译时，某类型的大小无法确定。但使用该类型时，上下文却要知道它确切的大小
- 当你有大量数据，想移交所有权，但需要确保在操作时数据不会被复制
- 使用某个值时，你只关心它是否实现了特定的trait，而不关心它的具体类型。

# 使用Box<T>在heap上存储数据
fn main() {
  let b = Box::new(5);
  println!("b = {}", b);
}

# 使用Box赋能递归类型
- 在编译时，Rust需要知道一个类型所占的空间大小
- 而递归类型的大小无法再编译时确定

- 但Box类型大小确定
  - 在递归类型中使用Box就可解决上述问题

- Box<T>是一个指针，Rust知道它需要多少空间，因为：
- 指针的大小不会基于它指向数据的大小变化而变化

- Box<T>:
- 只提供了 “间接” 存储和heap内存分配的功能
- 没有其他额外功能
- 没有性能开销
- 适用于间接存储场景
- 实现了 Deref trait 和 Drop trait

eg: 
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}