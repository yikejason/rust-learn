### Trait
- Trait 告诉 Rust 编译器:
  - 某种类型具有哪些并且可以与其它类型共享的功能
- Trait: 抽象的定义共享行为
- Trait bounds(约束)：泛型类型参数指定为实现了特定行为的类型
- Trait 与其它接口 （interface）类似，但有些区别

## 定义一个 Trait
- Trait的定义：把方法签名放在一起，来定义实现某种目的所必须得一组行为
  - 关键字：trait
  - 只有方法签名，没有具体实现 
  - trait可以有多个方法：每个方法签名占一行，以; 结尾
  - 实现该trait的类型必须提供具体的方法实现

eg:
pub trait Summary {
    fn summarize(&self) -> String;
    fn article(&self) -> String;
}

fn main() {}

## 在类型上实现trait
- 与为类型实现方法类似
- 不同之处：
  - impl XXX for Tweet {...}
  - 在 impl 的块里，需要对 Trait 里的方法签名进行具体的实现
lib.rs
eg: 

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

main.rs
eg:

use demo::Summary;
use demo::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize())
}

## 实现trait的约束
- 可以在某个类型上实现某个trait的前提条件是:
  - 这个类型 或 这个trait是在本地crate里定义的

- 无法为外部类型来实现外部的trait
  - 这个限制是程序属性的一部分 （也就是一致性）
  - 更具体地来说是孤儿原则：之所以这样命名是因为父类型不存在
  - 此规则确保其他人的代码不能破坏您的代码，反之亦然
  - 如果没有这个规则，两个crate可以为同一类型实现同一个trait，Rust就不知道应该使用哪一个实现了

## 默认实现
lib.rs
eg:
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more")  // 默认实现
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

main.rs
eg:
use demo::NewsArticle;
use demo::Summary;
use demo::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("This is a problem"),
        content: String::from("I love my wife"),
        author: String::from("jason"),
        location: String::from("ChengDu"),
    };

    println!(
        "1 new tweet: {}. 1 new article {}",
        tweet.summarize(),
        article.summarize()
    )
}

- 默认实现的方法可以调用trait中其他的方法，即使这些方法没有默认的实现
- 注意：无法从方法的重写实现里面调用默认的实现

## Trait 作为参数
- impl Trait语法：适用于简单情况
eg: 
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {} ...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

- Trait bound语法：可用于复杂情况
  - impl Trait 语法是 Trait bound的语法糖
eg:
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {} ...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary, item1: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound 写法
pub fn notify_two<T: Summary>(item: T, item1: T) {
    println!("Breaking news! {}", item.summarize());
}

- 使用 + 指定多个trait bound
eg:
use std::fmt::Display;

pub fn notify(item: impl Summary + Display) {
    println!("{}", item.summarize());
}

pub fn notify<T: Summary + Display>(item: T) {
    println!("{}", item.summarize());
}


- Where 语句简化泛型参数的trait
pub fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("{}", a.summarize())
}

- 简化后
pub fn notify<T, U>（a: T, b: U）-> String
     where T: Summary + Display,
           U: Clone + Debug,
{
   format!("{}", a.summarize())
} 

- Trait bound使用where子句
 - 在方法签名后指定where子句

## 实现 Trait 作为返回类型
- impl Trait 语法
  pub fn notify(s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("hello");
    }
  }

注意: impl Trait 只能返回确定的同一种类型，返回可能不同类型的代码会报错

下面代码会报错
pub fn notify(flag: bool) -> impl Summary {
    if bool {
        NewsArticle {
            headline: String::from("hello");
        }
    } else {
        Tweet {...}
    }
  }

## 使用 trait Bound 有条件的实现方法
- 在使用泛型类型参数的impl块上使用 Trait bound， 我们可以有条件的为实现了特定 Trait 的类型来实现方法


