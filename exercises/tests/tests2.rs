// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 两个相等的可比较值，断言成功，测试通过
        assert_eq!(42, 42);
        // 也可以用其他类型（如字符串、布尔值），只要满足 PartialEq 特质
        // assert_eq!("hello", "hello");
        // assert_eq!(true, !false);
    }
}
