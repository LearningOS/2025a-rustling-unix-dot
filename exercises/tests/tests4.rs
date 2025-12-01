// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 验证构造函数是否正确设置宽高
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 检查宽度
        assert_eq!(rect.height, 20); // 检查高度
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_width() {
        // 验证负宽度触发 panic（指定预期 panic 信息，让测试更精确）
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_height() {
        // 验证负高度触发 panic（指定预期 panic 信息，避免误判其他 panic）
        let _rect = Rectangle::new(10, -10);
    }
}
