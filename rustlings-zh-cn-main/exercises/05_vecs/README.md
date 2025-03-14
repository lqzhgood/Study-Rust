# 动态数组(Vectors)


动态数组(Vectors)是Rust中最常用的数据结构之一。在其他编程语言中，它们可能被简单地称为数组(Arrays)，但由于Rust在稍低的层级上进行操作，Rust中的数组存储在栈上(这意味着它不能增长或收缩，并且其大小需要在编译时确定)，而动态数组存储在堆上(在堆上这些限制并不适用)。

动态数组在书中属于稍靠后的章节内容，但我们认为它们非常有用，值得提前介绍一下。稍后我们还会讨论另一种有用的数据结构——哈希表(hash maps)。

## 对应知识

- [Storing Lists of Values with Vectors(使用动态数组存储值列表)](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`(可变迭代器)](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
