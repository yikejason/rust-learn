### 控制流

# if表达式
- let 语句中右侧使用 if表达式
if的每个分支中的返回值都必须是相同的类型
fn main () {
  let condition = true;
  let number = if condition { 5 } else { 6 };
  println("the value of number is: {}", number);
}

## 使用循环重复执行

# 使用loop重复执行
fn main () {
  loop {
    println!("again");
  }
}
break 中断loop循环

# while 条件循环
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

# 使用for遍历集合
fn main () {
  let a = [10, 20, 30, 40];
  for element in a {
    println!("the value is: {}", element);
  }
}

fn main() {
  for number in (1..4).rev() {
    println!("{}!", number);
  }
    println!("LIFTOFF!!!");
}


