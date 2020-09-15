use actix_web::{get,App, HttpServer};

/// ## Graceful shutdown
/// HttpServer 支持优雅关机. 在接收到停机信号后，worker线程有一定的时间来完成请求. 超过时间后的所有worker都会被强制drop掉.
/// 默认的shutdown 超时时间设置为30秒. 你也可以使用 HttpServer::shutdown_timeout() 方法来改变这个时间.
///
/// 您可以使用服务器地址向服务器发送停止消息，并指定是否要正常关机, server的start()方法可以返回一个地址.
///
/// HttpServer 处理几种OS信号. ctrl-c 在所有操作系统上都适用(表示优雅关机), 也可以在其它类unix系统上使用如下命令:
/// * SIGINT - 强制关闭worker
/// * SIGTERM - 优雅关闭worker
/// * SIGQUIT - 强制关闭worker
/// 另外也可以使用 HttpServer::disable_signals()方法来禁用信号处理
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .shutdown_timeout(60)// 设置关闭时间为60秒 超时后强制关闭worker
        .bind("127.0.0.1:8080")?
        .run().await
}

#[get("/index")]
async fn index() -> String {
    "Rust Graceful Shutdown Demo".to_string()
}