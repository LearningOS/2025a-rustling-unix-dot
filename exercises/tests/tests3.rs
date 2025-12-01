// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 测试偶数：调用 is_even 传入偶数（如 2、4、6 等），结果应为 true
        assert!(is_even(2));
        // 可选：多测试几个偶数，覆盖更多场景
        // assert!(is_even(4));
        // assert!(is_even(0)); // 0 是偶数，也应返回 true
    }

    #[test]
    fn is_false_when_odd() {
        // 测试奇数：调用 is_even(5)，结果应为 false，用 assert! 需取反
        assert!(!is_even(5));
        // 可选：多测试几个奇数，覆盖更多场景
        // assert!(!is_even(3));
        // assert!(!is_even(-1)); // 负数奇数也应返回 false
    }
}

