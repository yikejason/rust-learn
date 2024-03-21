### HashMap 
- hashmap<K,V>
- 键值对的形式存储数据，一个键 (key) 对应一个值 （value）
- Hash函数：决定如何在内存中存放 K 和 V
- 适用场景：通过 K （任何类型）来寻找数据，而不是通过索引

## 创建HashMap
- 创建空 HashMap: new()函数
- 添加数据：insert() 方法

use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 5);
}

- HashMap 用的较少，不在 Prelude 中
- 标准库对其支持较少，没有内置的宏来创建HashMap
- 数据存储在heap（堆）上
- 同构的。一个HashMap中
  - 所有的 K 必须是同一种类型
  - 所有的 V 必须是同一种类型

# 另一种创建 HashMap 的方式：collect 方法
- 在元素类型为Tuple的Vector上使用 collect 方法，可以组建一个 HashMap: 
  - 要求 Tuple 有两个值: 一个作为K，一个作为V
  - collect方法可以把数据整合成很多种集合类型，包括 HashMap
    - 返回值需要显示的指明类型

use std::collections::HashMap;

fn main() {
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

## HashMap 和所有权
- 对于实现了 Copy trait 的类型（例如i32），值会被复制到HashMap 中
- 对于拥有所有权的值 （例如 String），值会被移动，所有权会转移给 HashMap

## 访问 HashMap中的值
- get 方法
  - 参数：K
  - 返回: Option<&V>

use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);

  match scores {
    Some(s) => println!("{}", s),
    None => println!("team not exist"),
  }
}

# 遍历 HashMap
use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  for (k, v) in &scores {
    println!("{}: {}", k, v);
  }
}

## 更新 HashMap
- 替换现有的 V
use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 50);

  for (k, v) in &scores {
    println!("{}: {}", k, v);
  }
}

- 只在 K 不对应任何值的情况下，才插入V


  




