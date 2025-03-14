fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 数组(Array)

    // TODO: 创建一个名为 `v` 的动态数组(vector)，并且包含与数组 `a` 完全相同的元素。
    // 使用 vector 宏。
    // let v = ???;

    (a, v)
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
