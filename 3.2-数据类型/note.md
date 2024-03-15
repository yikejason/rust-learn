### 数据类型

- Rust 是一种静态类型的语言，这意味着它必须在编译期知道所有变量的类型

## 标量类型

- Rust 有 4 个基本的标量类型：整型、浮点型、布尔型和字符

# 整数类型

- 分为有符号类型和无符号类型
- （有符号整型以 i 开始，i 是英文单词 integer 的首字母，与之相反的是 u，代表无符号 unsigned 类型）
- u32 无符号类型 i32 有符号类型
- 有符号类型可以取负数， 无符号不行 
- 范围计算有符号类型和无符号类型  具体参考rust程序设计 https://rustwiki.org/zh-CN/book/ch03-02-data-types.html


# 整型溢出
假设有一个 u8 ，它可以存放从 0 到 255 的值。那么当你将其修改为范围之外的值，比如 256，则会发生整型溢出。关于这一行为 Rust 有一些有趣的规则：当在 debug 模式编译时，Rust 会检查整型溢出，若存在这些问题，则使程序在编译时 panic(崩溃,Rust 使用这个术语来表明程序因错误而退出)。

在当使用 --release 参数进行 release 模式构建时，Rust 不检测溢出。相反，当检测到整型溢出时，Rust 会按照补码循环溢出（two’s complement wrapping）的规则处理。简而言之，大于该类型最大值的数值会被补码转换成该类型能够支持的对应数字的最小值。比如在 u8 的情况下，256 变成 0，257 变成 1，依此类推。程序不会 panic，但是该变量的值可能不是你期望的值。依赖这种默认行为的代码都应该被认为是错误的代码。

- 要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
- 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add

下面是一个演示wrapping_*方法的示例：
为什么会输出 19
fn main() {
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19
}
解释：在 Rust 中，u8 类型的范围是 0 到 255。当你尝试对 255 进行 wrapping_add(20) 操作时，由于 u8 类型的范围是 0 到 255，因此会发生溢出。溢出后的结果会被 "包装" 回到该类型的范围内。

具体来说，255 + 20 会得到 275，但由于 u8 类型的范围是 0 到 255，因此会发生溢出，将 275 包装回 0，然后再加上剩余的 19，所以结果是 19。

这就是为什么你的代码输出 19。

- 如果使用 checked_* 方法时发生溢出，则返回 None 值
fn main() {
    let result = u8::MAX.checked_add(1); // 尝试将最大的 u8 数值加 1
    match result {
        Some(value) => println!("Result: {}", value), // 如果没有溢出，则打印结果
        None => println!("Overflow occurred!"), // 如果发生溢出，则打印溢出信息
    }
}
溢出了 这个会打印 Overflow occurred!

- 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
fn main() {
    let (result, overflowed) = u8::MAX.overflowing_add(1); // 尝试将最大的 u8 数值加 1
    if overflowed {
        println!("Overflow occurred! Result may not be accurate.");
    } else {
        println!("Result: {}", result);
    }
}
溢出了会打印 Overflow occurred! Result may not be accurate.

- 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值，例如:
assert_eq!(100u8.saturating_add(1), 101);
assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

# 浮点类型
- f32 f64 它们的大小分别为32位和64位 默认浮点类型是 f64（f64精度更高）
- 浮点类型都是有符号类型
- fn main () {
    let x = 2.0;  // f64
    let y: f32 = 3.0; // f32
  }

# 数字运算
- Rust 的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取模运算。整数除法会向下取整。

# 布尔类型
- 布尔值分别是 true 和 false
- fn main () {
    let t = true;
    let f: bool = false;
  }
- 条件判断if表达式中用布尔值居多

# 字符类型
- 用单引号引用  与字符串字面量（双引号）不同
- fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}


## 复合类型
- 可以将多个值组合成一个类型。Rust有两种基本的复合类型：元组(tuple)和数组(array)

# 元组类型
- 元组长度固定，一旦声明无法增长或缩小
- let tup: (i32, f64, u8) = (500, 6.4, 1);

- 获取元组中的值用模式结构
fn main () {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of y is {}", y);
}

- 除了上面的模式匹配外，还可以使用一个句点 . 连上要访问的值的索引来直接访问元组元素。
fn main () {
  let x: (i32, f64) = (400, 3.2);
  let first = x.0;
  let second = x.1;
}

- 没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 ()。该类型被称为单元类型（unit type），该值被称为单元值（unit value）。如果表达式不返回任何其他值，就隐式地返回单元值。

# 数组类型
- 数组每个元素必须有相同的类型，与其它语言不同 Rust中的数组具有固定长度
- 使用方括号编写数组的类型，其中包含每个元素的类型、分号，然后是数组中的元素数
let x: [i32; 5] = [1, 2, 3, 4, 5];

- 创建有相同值的数组，指定初始值，再跟分号，再跟数组长度
let x: [2; 5] = [2, 2, 2, 2, 2];

- 访问数组元素
fn main () {
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
}

- 无效的数组索引访问Rust程序会panic









