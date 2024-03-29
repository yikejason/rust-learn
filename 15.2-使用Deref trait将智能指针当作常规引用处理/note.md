### Deref Trait
- 实现 Deref Trait 使我们可以自定义解引用运算符 * 的行为
- 通过实现 Deref, 智能指针可像常规引用一样来处理

## 解引用运算符
- 常规的引用是一种指针
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

## 把 Box<T> 当作引用
- Box<T> 可以代替上例中的引用
eg:
fn main() {
  let x = 5;
  let y = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}

## 定义自己的智能指针
- Box<T>被定义成拥有一个元素的tuple struct
  eg: MyBox<T>

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

fn main() {
  let x = 5;
  let y = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}

## 实现 Deref Trait 
- 标准库中的 Deref trait 要求我们实现一个 deref 方法:
- 该方法借用self
eg: 
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

