fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// // 函数参数的类型必须进行标注。
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
