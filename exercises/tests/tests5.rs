// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
/// （安全契约：传入的 `address` 必须是一个指向有效 `u32` 值的可变内存地址，且调用者需保证该地址的独占访问权，避免数据竞争）
unsafe fn modify_by_address(address: usize) {
    // SAFETY: 遵循外层函数的安全契约——调用者已保证 `address` 是指向有效 `u32` 的可变地址，
    // 因此将 `usize` 地址转换为 `*mut u32` 指针是安全的，且解引用后可修改目标值。
    unsafe {
        // 1. 将 usize 地址转换为 *mut u32 可变原始指针
        let ptr = address as *mut u32;
        // 2. 解引用指针，修改目标内存的值为 0xAABBCCDD
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
