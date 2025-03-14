mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 在 `fn` 前添加 `pub` 使其在模块外部可被访问。
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
