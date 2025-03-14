// Clippy工具是一系列用于分析代码的lint(检查工具)，这样你就可以发现常见错误并改进你的Rust代码。
//
// 对于这些练习，当存在Clippy警告(Clippy warnings)时，代码将无法编译。
// 从输出中查看Clippy的建议来解决练习问题。

use std::f32::consts::PI;

fn main() {
    // 使用更精确的 `PI` 常量。
    let pi = PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
