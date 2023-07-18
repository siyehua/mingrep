use std::env;
use std::process;
use mingrep::{Config, run};

fn main() {
    // cargo run -- siyehua testfile.txt > output.txt
    // -- 可选参数
    // > 输入到指定的文件
    // env::args() 获取命令行输入的参数
    let inputs: Vec<String> = env::args().collect();
    let config = Config::build(&inputs).unwrap_or_else(|err| {
        // 使用 eprintln 替换 println, 就不会输出到文件中
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(&config).unwrap_or_else(|err|{
        eprintln!("Read file '{}' content error: {err}", &config.file_path);
        process::exit(1);
    });


}
