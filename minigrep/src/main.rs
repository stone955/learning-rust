use minigrep::{run, Config};
use std::{env, process};

fn main() {
    // 迭代1: 基本功能
    // 问题：
    // - 单一且庞大的函数。
    //      对于 minigrep 程序而言， main 函数当前执行两个任务：解析命令行参数和读取文件。但随着代码的增加，main 函数承载的功能也将快速增加。
    //      从软件工程角度来看，一个函数具有的功能越多，越是难以阅读和维护。因此最好的办法是将大的函数拆分成更小的功能单元。
    // - 配置变量散乱在各处。
    //      还有一点要考虑的是，当前 main 函数中的变量都是独立存在的，这些变量很可能被整个程序所访问，在这个背景下，独立的变量越多，越是难以维护，
    //      因此我们还可以将这些用于配置的变量整合到一个结构体中。
    // - 细化错误提示。
    //      目前的实现中，我们使用 expect 方法来输出文件读取失败时的错误信息，这个没问题，但是无论任何情况下，
    //      都只输出 Should have been able to read the file 这条错误提示信息，显然是有问题的，
    //      毕竟文件不存在、无权限等等都是可能的错误，一条大一统的消息无法给予用户更多的提示。
    // - 使用错误而不是异常。
    //      假如用户不给任何命令行参数，那我们的程序显然会无情崩溃，原因很简单：index out of bounds，一个数组访问越界的 panic，但问题来了，用户能看懂吗？
    //      甚至于未来接收的维护者能看懂吗？因此需要增加合适的错误处理代码，来给予使用者给详细友善的提示。还有就是需要在一个统一的位置来处理所有错误，利人利己！
    // {
    //     // 读取命令行参数
    //     let args: Vec<String> = env::args().collect();

    //     let query = &args[1];
    //     let file_path = &args[2];

    //     println!("Searching for {} from {}", query, file_path);

    //     // 读取文件
    //     let content =
    //         fs::read_to_string(file_path).expect("Should have been able to read the file");

    //     println!("With text:\n{}", content);
    // }

    // 迭代2: 增加模块化和错误处理
    // {
    //     // 读取命令行参数
    //     let args: Vec<String> = env::args().collect();

    //     // 分离命令行解析
    //     // let config = parse_config(&args);
    //     // 聚合配置变量
    //     // let config = Config::from(&args);
    //     // 处理返回的 Result
    //     let config = Config::build(&args).unwrap_or_else(|err| {
    //         // 重定向错误信息的输出
    //         eprintln!("Problem parsing arguments: {}", err);
    //         process::exit(1);
    //     });

    //     println!(
    //         "Searching for {} from {}",
    //         config.query,
    //         config.file_path
    //     );

    //     // 分离主体逻辑, 处理返回的错误
    //     if let Err(err) = run(config) {
    //         // 重定向错误信息的输出
    //         eprintln!("Application error: {}", err);
    //         process::exit(1);
    //     }
    // }

    // 迭代3: 使用迭代器改进程序
    {
        let config = Config::build(env::args()).unwrap_or_else(|err| {
            // 重定向错误信息的输出
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        println!("Searching for {} from {}", config.query, config.file_path);

        // 分离主体逻辑, 处理返回的错误
        if let Err(err) = run(config) {
            // 重定向错误信息的输出
            eprintln!("Application error: {}", err);
            process::exit(1);
        }
    }
}
