// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // 变体携带一个包含x和y字段的结构体
    Move { x: i32, y: i32 },
    // 变体携带一个String类型的值
    Echo(String),
    // 变体携带三个i32类型的值（分别表示RGB颜色分量）
    ChangeColor(i32, i32, i32),
    // 单元变体，不携带任何数据
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

