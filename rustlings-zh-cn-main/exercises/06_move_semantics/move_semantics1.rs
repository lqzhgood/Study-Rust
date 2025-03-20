// TODO: 修改这个函数中的编译器错误。
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    /*************  ✨ Codeium Command ⭐  *************/
    /// Tests that `fill_vec` takes ownership of its argument and returns a new vector
    /// with the added element.
    /******  c6696515-9c1a-4afa-b07c-ea53c9696a9b  *******/
    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
