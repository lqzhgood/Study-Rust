fn main() {
    let number = "T-H-R-E-E";
    println!("拼写number: {}", number);

    // 使用变量遮蔽(variable shadowing)
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let number = 3;
    println!("number加上2等于: {}", number + 2);
}
