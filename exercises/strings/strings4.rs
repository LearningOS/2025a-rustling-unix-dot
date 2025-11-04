// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // "blue" 是字符串字面量，类型为 &str
    string("red".to_string()); // to_string() 将 &str 转为 String
    string(String::from("hi")); // String::from 直接创建 String
    string("rust is fun!".to_owned()); // to_owned() 为 &str 创建拥有所有权的 String
    string("nice weather".into()); // into() 自动转换为 String（基于类型推断）
    string(format!("Interpolation {}", "Station")); // format! 宏返回 String
    string_slice(&String::from("abc")[0..1]); // 切片操作返回 &str，即使原类型是 String
    string_slice("  hello there ".trim()); // trim() 方法返回 &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace() 返回 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // to_lowercase() 返回 String
}
