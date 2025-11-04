// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // 使用 str 的 trim() 方法去除两端空格，再转换为 String
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // 方法1：使用 format! 宏拼接字符串（推荐，语法简洁）
    format!("{} world!", input)
    
    // 方法2：使用 String 的 push_str 方法（需先将 &str 转为 String）
    // let mut result = input.to_string();
    // result.push_str(" world!");
    // result
}

fn replace_me(input: &str) -> String {
    // 使用 str 的 replace() 方法替换子串，直接返回 String
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
