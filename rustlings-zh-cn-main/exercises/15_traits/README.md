# 特征(Traits)

特征(Trait)是一组方法的集合。

数据类型可以实现特征。要实现特征，就需要为该数据类型定义组成特征的各个方法。例如，`String` 数据类型实现了 `From<&str>` 特征，这使得用户可以编写 `String::from("hello")` 这样的代码。

通过这种方式，特征在某种程度上与Java中的接口(interfaces)以及C++中的抽象类(abstract classes)类似。

另外一些常见的Rust特征包括:
- `Clone`(具有 `clone` 方法)
- `Display`(允许通过 `{}` 进行格式化显示)
- `Debug`(允许通过 `{:?}` 进行格式化显示)

由于特征表明了数据类型之间共有的行为，所以在编写泛型时它们很有用处。 

## 对应知识

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
