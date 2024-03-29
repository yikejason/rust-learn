### 所有可能会用到模式的位置
# match 分支
- match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

# if let 条件表达式
- eg:
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color is {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

# while let 条件循环
- eg:
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

# for 循环
- eg:
fn main() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
- 这里使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引，他们位于一个元组中。第一个 enumerate 调用会产生元组 (0, 'a')。当这个值匹配模式 (index, value)，index 将会是 0 而 value 将会是 'a'，并打印出第一行输出。


# let 语句 也是一种匹配模式
let PATTERN = EXPRESSION;

let (x, y, z) = (1, 2, 3);
let (x, y, _) = (1, 2, 3);

let (x, y) = (1, 2, 3);   会报错不能匹配上

# 函数参数也是模式
- eg:
fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({},{})", x, y);
}

# 闭包类似于函数，也可以在闭包参数列表中使用模式


###  模式是否会匹配失效

- irrefutable：能匹配任何传递的可能值的模式  （不可反驳模式）
eg: let x = 5;

- refutable : 对某些可能的值进行匹配会失败的模式  （可反驳模式）
eg:
一个这样的例子便是 if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。


### 模式语法

# 匹配字面量
- fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

# 匹配命名变量
eg:
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

输出：
Matched, y = 5
at the end: x = Some(5), y = 10

# 多个模式
- 在match表达式中 可以使用 | 语法匹配多个模式
eg:
fn main() {
    let x = 1;
    
    match x {
        1 | 2 => println!(),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

# 通过 ..= 匹配值的范围
eg:
fn main() {
    let x = 5;
    let y = 'c';
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    match y {
        'a'..='j' => println!("early test"),
        _ => println!("something else"),
    }
}

## 解构并分解值
- 可以使用模式来解构结构体、枚举、元组和引用，以便使用这些值的不同部分。让我们来分别看一看。

# 解构结构体

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

- 优化一下
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

- eg: 这个例子会匹配到第二个分支 On the y axis at 7
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({},{})", x, y),
    }
}

# 解构枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // let msg = Message::Quit;
    // let msg = Message::Move { x: i32, y: i32};
    // let msg = Message::Write(String::from("hello"));
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}
- 这段代码会打印出 Change the color to red 0, green 160, and blue 255。

# 解构嵌套的结构体和枚举
eg:
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to red {}, green {}, and blue {}", h, s, v)
        }
        _ => (),
    }
}

# 解构结构体和元组
eg:
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!(
        "feet is {}, inches is {}, x is {}, y is {}",
        feet, inches, x, y
    )
}

feet is 3, inches is 10, x is 3, y is -10

# 忽略模式中的值
- 使用 _ 忽略值
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}

- 使用嵌套 _ 忽略部分值
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

- 在模式中多次使用 _ 来忽略某值
fn main() {
    let number = (2, 4, 8, 16, 32);
    match number {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

Some numbers: 2, 8, 32

- 通过在名字前以一个下划线开头来忽略未使用的变量
fn main() {
  let _x = 1;
}

- 用 .. 忽略剩余值
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

- 亦可以忽略中间的值
fn main() {
  let numbers = (1, 2, 3, 4, 5);
  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    }
  }
}

# 匹配守卫的额外条件
- if x < 5 就是额外条件
fn main() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}
- 匹配守卫覆解决变量覆盖问题
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

- 组合多个模式的匹配守卫
fn main() {
  let x = 4;
  let y = false

  match x {
    4 | 5 | 6 if y => println("yes"),
    _ => println!("no"),
  }
}

# @绑定
- 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
enum Message {
    Hello { id: i32 },
}
fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable);
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
