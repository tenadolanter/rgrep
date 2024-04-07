use std::env;
use std::fs;
fn main() {
    //  cargo run -- name 111.txt
    // 获取参数
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("{}{}", query, file_path);

    // 读取文件内容
    let content = fs::read_to_string(file_path).expect("文件读取");
    println!("text content:\n{content}")

}
