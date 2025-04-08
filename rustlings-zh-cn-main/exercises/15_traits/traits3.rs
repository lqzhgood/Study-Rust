trait Licensed {
    // TODO: 为 `licensing_info` 添加一个默认实现，
    // 以便像下面这两个结构体这样的实现者能够共享该默认行为，而无需重复编写该函数。
    // 默认许可信息应该为字符串 "Default license"。
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // 不要编辑此行代码。
impl Licensed for OtherSoftware {} // 不要编辑此行代码。

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
