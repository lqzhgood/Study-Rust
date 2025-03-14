// TODO: 在不将宏的定义语句移出这个模块的情况下修复编译器错误。
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
