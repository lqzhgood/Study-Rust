fn trim_me(input: &str) -> &str {
    // TODO: 去除字符串两端的空白字符。
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: 在字符串后面添加 "world!" ，有很多方法可以做到这一点。
    let mut a = input.to_string();
    a.push_str(" world!");
    a
}

fn replace_me(input: &str) -> String {
    // TODO: 替换字符串中的 "cars" 为 "balloons" 。
    input.replace("cars", "balloons")
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
