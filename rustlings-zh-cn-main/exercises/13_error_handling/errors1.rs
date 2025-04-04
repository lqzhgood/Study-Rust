// TODO: 如果你给这个函数传入一个空字符串，它将拒绝生成要打印在名牌(nametag)上的文本。
// 如果它能解释问题所在，而不仅仅是返回 `None`，那就更好了。
// 幸运的是，Rust有一个与 `Option` 类似的结构，可以用来表示错误情况。
// 将函数签名和函数体改为返回 `Result<String, String>`，而不是 `Option<String>`。

fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 不允许姓名为空。
        Err("不允许姓名为空".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("不允许姓名为空"),
        );
    }
}
