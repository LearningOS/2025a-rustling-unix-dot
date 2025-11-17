// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// 先定义宏
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 后使用宏
    my_macro!();
}
