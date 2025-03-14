// 这是针对以下章节内容的一个小测验:
// - 变量
// - 函数
// - 条件判断(if语句)
//
// 小明正在买苹果。一个苹果的价格按如下方式计算：
// - 一个苹果售价2r(r: Rust元)。
// - 如果购买的苹果数量超过40个，那么每个苹果的价格变为1r!

// TODO: 编写一个函数，根据购买的苹果数量来计算总价。
// fn calculate_price_of_apples(???) -> ??? { ??? }
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
