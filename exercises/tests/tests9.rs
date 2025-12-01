extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    // 为别名函数添加 link_name 属性，绑定到实际函数的符号
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // 添加 no_mangle 属性，禁止符号混淆，让外部能按原名找到该函数
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
