fn current_favorite_color() -> String {
    // 等同于 `String::from("blue")`
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("我目前喜欢的颜色是 {answer}");
}
