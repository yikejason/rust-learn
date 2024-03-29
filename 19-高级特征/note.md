### 高级特征

## 不安全的Rust   以后进阶再学
- unsafe
- 不安全的超能力
1. 解引用裸指针
2. 调用不安全的函数或方法
3. 访问或修改可变静态变量
4. 实现不安全 trait
5. 访问 union 的字段

# 解引用裸指针
eg:
fn main() {
  let mut num = 5;
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;
}

## 高级 trait
# 关联类型在 trait 定义中指定占位符类型


## 高级类型

## 高级函数与闭包

## 宏

