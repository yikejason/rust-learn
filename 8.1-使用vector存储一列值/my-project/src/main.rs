enum Spread {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    // 创建 Vector
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    // 更新 Vector
    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // println!("{:?}", v);

    // 读取 Vector的元素
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element"),
    // }

    // 所有权和借用规则
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is {}", first);

    // 遍历vector
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i)
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }

    // for i in v {
    //     println!("{}", i);
    // }
    let row = vec![
        Spread::Int(3),
        Spread::Text(String::from("blue")),
        Spread::Float(10.12),
    ]
}
