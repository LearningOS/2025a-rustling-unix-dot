// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 修复：避免在知道是 None 时调用 unwrap（会 panic）
    if my_option.is_some() {
        my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3,  // 修复：添加逗号分隔数组元素
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修复：resize 方法返回 ()，应先创建向量再修改
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 修复：使用 std::mem::swap 正确交换两个变量的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
