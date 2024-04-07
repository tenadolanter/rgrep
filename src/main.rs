use rgrep::Config;
use std::env;
use std::process;
fn main() {
    //  cargo run -- name 111.txt

    // 获取参数
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(1);
    });

    // 读取文件内容
    if let Err(e) = rgrep::run(config) {
        eprintln!("error:{}", e);
        process::exit(1);
    }
}
