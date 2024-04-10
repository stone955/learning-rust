use std::{env, fs};

fn main() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {} from {}", query, file_path);

    // 读取文件
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{}", content);
}
