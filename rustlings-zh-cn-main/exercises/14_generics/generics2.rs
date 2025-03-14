// 这个强大的包装器(wrapper)，它具备存储正整数值的能力。
// TODO: 使用泛型重写它，使得它能支持包装任何类型。
struct Wrapper {
    value: u32,
}

// TODO: 调整结构体的实现(impl)，使其对于被包装的值是泛型的。
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
   // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
