#![allow(clippy::ptr_arg)]

// 借用(Borrows)而不是获取所有权(ownership)。
// 在这里，建议使用 `&str` 而不是 `&String`。但目前这样就ok，因为我们还没有处理字符串相关的内容。
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 获取所有权，而不是借用。
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust真不错!".to_string();

    get_char(&data);

    string_uppercase(data);
}
