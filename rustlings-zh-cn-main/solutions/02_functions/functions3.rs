fn call_me(num: u8) {
    for i in 0..num {
        println!("叮! 调用数字 {}", i + 1);
    }
}

fn main() {
    // 函数 `call_me` 需要传入一个参数。
    call_me(5);
}
