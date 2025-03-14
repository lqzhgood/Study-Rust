// 使用位运算中的左移操作来计算2的幂。
// `1 << n` 等同于 `2的n次幂`。
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(power_of_2(0), 1);
        assert_eq!(power_of_2(1), 2);
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(3), 8);
    }
}
