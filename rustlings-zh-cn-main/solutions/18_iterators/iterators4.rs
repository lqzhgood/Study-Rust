// 这里展示三种可能的解决方案。

// 使用 `for` 循环 和 一个可变变量。
fn factorial_for(num: u64) -> u64 {
    let mut result = 1;

    for x in 2..=num {
        result *= x;
    }

    result
}

// 等同于 `factorial_for`，但更简短，且没有显式的 `for` 循环和可变变量。
fn factorial_fold(num: u64) -> u64 {
    // 当 num==0: 迭代器 2..=0 为空
    //            -> 返回 `fold` 的初始值，即1。
    // 当 num==1：迭代器2..=1也为空
    //            -> 返回初始值1。
    // 当 num==2：迭代器2..=2包含一个元素
    //            -> 初始值1乘以2并返回结果。
    // 当 num==3：迭代器2..=3包含2个元素
    //            -> 先计算1 * 2，然后结果2乘以第二个元素3，所以返回结果6。
    // 依此类推...
    #[allow(clippy::unnecessary_fold)]
    (2..=num).fold(1, |acc, x| acc * x)
}

// 等同于 `factorial_fold`，不过使用了Clippy建议的内置方法(built-in method)。
fn factorial_product(num: u64) -> u64 {
    (2..=num).product()
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial_for(0), 1);
        assert_eq!(factorial_fold(0), 1);
        assert_eq!(factorial_product(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial_for(1), 1);
        assert_eq!(factorial_fold(1), 1);
        assert_eq!(factorial_product(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial_for(2), 2);
        assert_eq!(factorial_fold(2), 2);
        assert_eq!(factorial_product(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial_for(4), 24);
        assert_eq!(factorial_fold(4), 24);
        assert_eq!(factorial_product(4), 24);
    }
}
