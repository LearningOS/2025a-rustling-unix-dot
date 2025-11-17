// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    // 使用 if let 替代 for 循环处理 Option，更符合 Rust 惯用写法
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
