### Result枚举
enum Result<T,E> {
  Ok(T),
  Err(E),
}

T: 操作成功， Ok变体里返回的数据的类型
E: 操作失败，Err变体里返回的错误的类型
