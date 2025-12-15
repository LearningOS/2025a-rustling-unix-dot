// clippy3.rs
// 
// Here's  a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// 移除不必要的 allow 注解（修复后不再需要）
fn main() {
    // 1. 标记未使用变量 + 明确 Option 类型（消除 unused_variable 和 冗余检查警告）
    let _my_option: Option<()> = None;

    // 2. 移除数组末尾冗余逗号（Clippy 提示 trailing_comma 警告）
    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // 3. 空 Vec 使用最简写法 vec![]（替代 Vec::new() 或其他冗余写法）
    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 4. 使用 std::mem::swap 替代手动临时变量交换（Clippy 推荐的惯用写法）
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
