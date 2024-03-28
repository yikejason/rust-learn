### 闭包
1. 闭包可以是保存进变量或者作为参数传递给其他函数的匿名函数。闭包和函数不同的是，闭包允许捕获调用者作用域中的值
- 闭包语法
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x| { x + 1};
  let add_one_v4 = |x| x + 1;

2. 闭包的使用方式
3. 使用带有泛型 和 Fn Trait的闭包


## eg
fn main() {
    let use_closure = || {
        println!("This is a closure");
    };
    use_closure();
    // 闭包定义会为每个参数和返回值类型推导一个具体的类型，但是不能推导两次
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1};
    let add_one_v4 = |x| x + 1;

    let a = add_one_v1(5); // use function
    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let c = add_one_v4(5);


    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);

    // 不能推导两次的闭包类型的例子  
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("s = {}", s);

    let n = example_closure(5);
    println!("n = {}", n);


    // 捕捉环境中的变量 上下文的值
    let i = 1;
    let exe = |x| x + i;
    let r = exe(5);
    println!("r = {}", r);
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}