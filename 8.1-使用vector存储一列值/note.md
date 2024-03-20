### 使用 Vector 存储多个值
- Vec<T>，叫做 vector
- 由标准库提供
- 可存储多个值
- 只能存储相同类型的数据
- 值在内存中连续存放

## 创建Vector
- Vec::new 函数
eg:

fn main() {
  let v:Vec<i32> = Vec::new();
}

- 通常使用初始值创建 Vec<T>, 使用vec!宏

## 更新Vector
- 向Vector添加元素，使用push方法


## 读取 Vector的元素
- 索引
- get方法

## 索引 vs get 处理访问越界
- 索引：panic
- get：返回None

## 所有权和借用规则
- 不能在同一作用域内拥有可变和不可变引用

## 遍历 Vector中的值
- for 循环

## 使用enum来存储多种数据类型
- Enum 的变体可以附加不同类型的数据
- Enum 的变体定义在同一个enum类型下