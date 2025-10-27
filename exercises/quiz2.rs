// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

// I AM NOT DONE

pub enum Command {
    Uppercase,  // 大写转换命令
    Trim,       // 去除空格命令
    Append(usize),  // 追加命令，携带追加次数
}

mod my_module {
    use super::Command;  // 引入父模块的Command枚举

    // 函数签名：输入是(String, Command)元组的向量，输出是String的向量
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];  // 初始化输出向量
        for (string, command) in input {  // 遍历输入的每个元组
            match command {  // 根据命令类型匹配处理逻辑
                // 1. 处理大写命令：调用String的to_uppercase()方法
                Command::Uppercase => output.push(string.to_uppercase()),
                // 2. 处理去除空格命令：先trim()去空格，再转成String（trim()返回&str）
                Command::Trim => output.push(string.trim().to_string()),
                // 3. 处理追加命令：先重复"bar"n次，再和原字符串拼接
                Command::Append(n) => {
                    let bar_repeated = "bar".repeat(n);  // 生成重复n次的"bar"
                    output.push(format!("{}{}", string, bar_repeated));  // 拼接并加入输出
                }
            }
        }
        output  // 返回处理后的字符串向量
    }
}

#[cfg(test)]
mod tests {
    // 导入my_module中的transformer函数，以及父模块的Command枚举
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        // 构造测试输入：每个元组是（原始字符串，命令）
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),  // 测试大写：hello → HELLO
            (" all roads lead to rome! ".into(), Command::Trim),  // 测试去空格：前后空格去除
            ("foo".into(), Command::Append(1)),  // 测试追加1次：foo → foobar
            ("bar".into(), Command::Append(5)),  // 测试追加5次：bar + 5个bar → 共6个bar
        ]);
        // 验证每个输出是否符合预期
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
