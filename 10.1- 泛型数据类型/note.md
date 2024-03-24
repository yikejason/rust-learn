## 重复代码

# 重复代码的危害
  - 容易出错
  - 需求变更时需要在多处进行修改

# 消除重复的步骤
- 识别重复的代码
- 提取重复代码到函数体中，并在函数签名中指定函数的输入和返回值。

eg:
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![34, 50, 25, 100, 6000];
    let result = largest(&number_list);

    println!("The largest number is {}", result);
}

## 泛型
- 泛型：提高代码复用能力
  - 处理重复代码的问题
- 泛型是具体类型或其他属性的抽象的代替
  - 你编写的代码不是最终的代码，而是一种模版，里面有一些占位符。
  - 编译器在编译时将占位符替换为具体的类型
  eg: 
  fn largest<T>(list: &[T]) -> T {...}
- 类型参数
- CamelCase
- T: type 的缩写

## 函数定义中使用泛型
- 泛型函数
  - 参数类型
  - 返回类型
  eg:  伪代码
  fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['h', 'l'];
    let result = largest(&number_list);

    println!("The largest number is {}", result);
}

## Struct 定义中的泛型

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 1, y: 1 };
    let test_string = Point { x: 1, y: 2.0 };
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 1, y: 1 };
    let test_string = Point { x: 1, y: 2 };
}

- 可以使用多个泛型的类型参数
  - 太多类型的参数：你的代码需要重组为多个更小的单元

## Enum 定义中的泛型
- 可以让枚举的变体持有泛型数据类型
  - 例如 Option<T>, Result<T,E>
eg:
enum Option<T> {
  Some(T),
  None,
}

enum Result<T,E> {
  Ok(T),
  Err(E),
}

## 方法定义中的泛型
- 为 struct 或 enum 实现方法的时候，可在定义中使用泛型
eg:
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {} {}", p.x(), p.y());
}

- 针对具体的类型写方法  例如i32类型
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

- 注意：
 - 把T放在impl关键字后，表示在类型T上实现方法
   impl<T> Point(T) {...}
 - 只针对具体类型实现方法 （其余类型没实现方法）
   impl Point<i32>

- struct 里的泛型参数可以和方法的泛型类型参数不同
eg:
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 4 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


## 泛型代码的性能
- 使用泛型的代码和使用具体类型的代码运行速度是一样的
- 单态化 (monomorphization)
  - 在编译时将泛型替换为具体类型的过程

    