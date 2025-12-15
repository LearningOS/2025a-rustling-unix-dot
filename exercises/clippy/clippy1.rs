// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::f32::consts::PI;

fn main() {
    // 1. 移除冗余的 f32 后缀（字面量 5.00 默认为 f64，需显式指定 f32 但可简化写法）
    let radius = 5.00_f32; 
    // 2. 使用 powi(2) 替代手动乘法（Clippy 推荐的平方写法）
    let area = PI * radius.powi(2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    ); // 3. 补充分号（Clippy 会提示语句结束缺少分号）
}
