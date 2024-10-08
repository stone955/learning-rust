use futures::{SinkExt, StreamExt};
use log::{error, info};
use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

// TcpListener 和 TcpStream 用于创建 TCP 监听器和连接。
// accept_async 用于接受 WebSocket 连接。
// Message 用于表示 WebSocket 消息。
// StreamExt 和 SinkExt 用于处理异步流和发送数据。
// env 用于访问环境变量，例如命令行参数。
// SocketAddr 用于表示网络地址。
// info 和 error 用于记录信息和错误消息。

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    env_logger::init();

    // 获取要绑定的地址
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // 解析地址
    let addr: SocketAddr = addr.parse().expect("invalid address");

    // 创建 TCP 监听器
    let listener = TcpListener::bind(&addr).await.expect("failed to bind");

    info!("listening on: {}", addr);

    // 循环处理传入的连接
    while let Ok((stream, _)) = listener.accept().await {
        // 为每个连接启动一个新的任务
        tokio::spawn(handle_connection(stream));
    }

    // 以上代码实现了以下功能：
    // 初始化日志记录器: 使用 env_logger::init() 初始化日志记录器，以便将信息和错误输出到控制台。
    // 获取地址: 从命令行参数获取要绑定的地址，如果没有提供，则默认使用 127.0.0.1:8080。
    // 创建 TCP 监听器: 使用 TcpListener::bind 创建一个 TCP 监听器，监听指定的地址。
    // 记录信息: 打印一条信息，表明服务器正在监听指定的地址。
    // 循环接受连接: 使用 listener.accept 循环接受来自客户端的连接，并将每个连接交给 handle_connection 函数处理。
    // 启动任务: 使用 tokio::spawn 为每个连接启动一个新的异步任务，以便同时处理多个连接。
}

// handle_connection 实现以下功能：
// 接受 WebSocket 连接: 使用 accept_async 尝试从 TCP 流中接受一个 WebSocket 连接，如果成功，则返回一个 WebSocket 流。
// 拆分流: 将 WebSocket 流拆分为一个发送器和一个接收器，分别用于发送和接收消息。
// 处理消息: 循环接收来自客户端的消息，并根据消息类型进行不同的处理：
// 文本消息:  反转接收到的文本内容，并将反转后的文本发送回客户端。
// 关闭消息:  结束连接。
// 其他消息类型:  忽略。
// 错误:  记录错误并结束连接。
async fn handle_connection(stream: TcpStream) {
    // 接受 WebSocket 连接
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("error during the WebSocket handshake: {}", e);
            return;
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();

    // 处理来自客户端的消息
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // 反转接收到的字符串并返回给客户端
                let reversed = text.chars().rev().collect::<String>();
                if let Err(e) = sender.send(Message::Text(reversed)).await {
                    error!("error sending message: {}", e);
                }
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => (),
            Err(e) => {
                error!("error processing message: {}", e);
                break;
            }
        }
    }
}
