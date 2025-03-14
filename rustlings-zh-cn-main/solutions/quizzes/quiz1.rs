// 小明正在买苹果。一个苹果的价格按如下方式计算：
// - 一个苹果售价2r(r: Rust元)。
// - 如果购买的苹果数量超过40个，那么每个苹果的价格变为1r!

fn calculate_price_of_apples(n_apples: u64) -> u64 {
    if n_apples > 40 {
        n_apples
    } else {
        2 * n_apples
    }
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

// 不要修改下面的测试!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
