### Slice类型
Slice（切片）类型，表示引用集合中一段连续的元素序列。Slice是一类引用，没有所有权。Rust常见类型中，有三种支持Slice的操作，分别是String、数组、Vec类型。

# Slice类型
1. s[n1..n2]: 获取s中index=n1 到index=n2 (不包括index=n2)之间的所有元素；
2. s[n1..]: 获取s中index=n1 到最后一个元素之间的所有元素
3. s[..n2]: 获取s中第一个元素到index=2(不包括index=n2) 之间的元素
4. s[..]: 获取s中的所有元素
5. 其他表示包含范围的方式，如s[n1..=n2]表示取index=n1到index=n2(包括n2)之间的所有元素。

Rust中几乎总是使用切片数据的引用。切片数据的引用对应的数据类型描述为&[T]或&mut [T]，前者不可通过Slice引用来修改源数据，后者可修改源数据。示例如下

fn main() {
  let mut arr = [11, 22, 33, 44];
  let arr_slice1 = &arr[..=1];
  println!("{:?}", arr_slice1); // [11,22];

  let arr_slice2 = &mut arr[..1];
  arr_slice2[0] = 1111;
  println!("{:?}", arr_slice2);// [1111,22];
  println!("{:?}", arr);// [1111,22,33,44];
}

Slice类型是一个指针，包含两个字段
1. 第一个字段是指向源数据起点的指针
2. 第二个字段是切片数据中包含的元素数量，即切片的长度。


# String的切片类型
- String的切片类型是 &str 而不是 &String
fn main() {
  let s = String::from("hello world");
  let s1 = &s[6..];    // 切片类型&str
  let s2 = &s;         // 引用类型&string
  println!("{:?}", s1);
  println!("{:?}", s2);
}

&str 和 &String内容类型 见这个地址图例: https://rustycab.github.io/LearnRustEasy/chapter_3/chapter_3_7_3.html


# 其他Slice
- 数组slice
fn main() {
  let a: [u32, 5] = [1, 2, 3, 4, 5];
  let b = &a[1..3];
  println!("b: {:?}", b);
}  // b: [2, 3]

- Vec的Slice
fn main() {
  let v: Vec<u32> = vec![1, 2, 3, 4, 5];
  let b = &v[1..3];
  println!("b: {:?}", b);
} // b: [2, 3]

