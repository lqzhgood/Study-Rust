# Options

`Option` 类型表示一个可选值: 每个 `Option` 要么是 `Some` 并包含一个值，要么是 `None` 且不包含值。

在Rust代码中，`Option` 类型非常常见，因为它们有许多用途:
- 初始值(Initial values)
- 对于在整个输入范围上未定义的函数(部分函数)的返回值
- 用于在发生错误时返回 `None` 以报告简单错误的返回值
- 可选的结构体字段
- 可以被借用或“获取”的结构体字段
- 可选的函数参数
- 空指针(Nullable pointers)
- 当遇到一些复杂或错误的情况，临时替换某个值或者表示该值无效

## 对应知识

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
