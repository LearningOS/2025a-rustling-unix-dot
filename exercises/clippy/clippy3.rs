// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// 移除不必要的 allow 注解（修复后不再需要）
fn main() {
    // 1. 移除冗余的 Option 检查（my_option 明确为 None，is_some() 永远为 false）
    let _my_option: Option<()> = None; // 加下划线标记未使用，避免 Clippy 警告

    // 2. 简化数组写法（移除多余逗号，Clippy 会提示 trailing commas 冗余）
    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // 3. 空 Vec 可直接用 vec![] 宏（更简洁，Clippy 推荐写法）
    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 4. 变量交换可用更直观的 std::mem::swap（或解构赋值，Clippy 无异议）
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
