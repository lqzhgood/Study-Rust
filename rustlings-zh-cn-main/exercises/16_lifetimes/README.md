# 生命周期(Lifetimes)

生命周期(lifetimes)告诉编译器如何检查引用是否存活足够长的时间，以便在任何给定情况下都有效。例如，生命周期会说“确保参数 'a' 与参数 'b' 存活时间一样长，这样返回值才有效”。

它们仅在借用(即引用)时是必要的，因为复制的参数或移动操作在其作用域内是拥有所有权的，无法在外部被引用。生命周期意味着函数的调用代码可以被检查，以确保其参数有效。生命周期对其调用者有限制作用。

如果你想了解更多关于生命周期标注(lifetime annotations)的内容，[lifetimekata](https://tfpk.github.io/lifetimekata/) 项目有与Rustlings类似风格的练习，但全是关于学习编写生命周期标注的。

## 对应知识

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes(使用生命周期验证引用)](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
