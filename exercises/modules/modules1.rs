// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

mod sausage_factory {
    // 保持此函数为私有，不允许模块外部访问
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 声明为公共函数，允许模块外部调用
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
