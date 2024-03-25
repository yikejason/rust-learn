### Rust中错误处理的操作
- 可恢复的错误
Result<T, E>

- 不可恢复的错误
!panic

### 使用panic产生的回溯信息
- panic! 可能出现在：
  1. 我们写的代码中
  2. 我们所依赖的代码中

### 匹配不同的错误
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };
}
上例中使用了很多match...
match 很有用  但是很原始
闭包 （closure）Result<T, E> 有很多方法
- 它们接收闭包为参数
- 使用match实现
- 使用这些方法会让代码更简洁

改写上面的代码
use std::{fs::File, io::ErrorKind};

fn main() {
    File::open("how.txt").unwrap_or_else(|error| {
        if (error.kind()) == ErrorKind::NotFound {
            File::create("how.txt").unwrap_or_else(|error| {
                panic!("Error creating file {:?}", error);
            })
        } else {
            panic!("Error opening file {:?}", error);
        }
    });
}

## unwrap 表达式中的快捷方法  不能自定义panic!中的信息
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").unwrap("this is a question");
}

## expect 自定义panic中所带的错误信息
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("this is a question");
}

### 传播错误
use std::fs::File;
use std::io::{Error, Read};

fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let result = read_username_from_file();
}

# 运算符 简化上面代码
- ？运算符 传播错误的一种快捷方式
- 如果 Result是Ok，Ok中的值就是表达式的结果，然后继续执行
- 如果 Result是Err，Err 就是整个函数的返回值，就像函数的return

use std::fs::File;
use std::io::{Error, Read};

fn read_username_from_file() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let result = read_username_from_file();
    print!("{:?}", result);
}