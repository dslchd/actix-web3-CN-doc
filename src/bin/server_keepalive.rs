use actix_web::{web, App, HttpServer, HttpResponse};

/// ## Keep-Alive
/// Actix 可以在keep-alive 链接上等待请求.
///
/// keep alive 链接行为被 server设置定义.
/// * 75, Some(75), KeepAlive::Timeout(75) - 开启keep alive 保活时间
/// * None or KeepAlive::Disable - 关闭keep alive设置
/// * KeepAlive::Tcp(75) - 使用 tcp socket SO_KEEPALIVE 设置选项
/// 如果第一下选项被选择,那么keep alive 状态将会根据响应的connection-type类型来计算.默认的 HttpResponse::connection_type没有被定义
/// 这种情况下会根据 http的版本来默认是否开启keep alive
///
/// keep alive 在 HTTP/1.0是默认关闭的，在HTTP/1.1和HTTP/2.0是默认开启的.
///
/// 链接类型可以使用 HttpResponseBuilder::connection_type() 方法来改变.
/// ```rust
///  use actix_web::{http, HttpRequest, HttpResponse};
///  async fn index(req: HttpRequest) -> HttpResponse {
///     HttpResponse::Ok().connection_type(http::ConnectionType::Close) // 关闭链接
///     .force_close() // 这种写法与上面那种写法二选一
///     .finish()
/// }
/// ```
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let one = HttpServer::new(||{
        App::new().route("/", web::get().to(|| HttpResponse::Ok().body("Ok")))
    }).keep_alive(75); // 设置keep alive 时间为75秒
    // let _two = HttpServer::new(||{
    //     App::new().route("/", web::get().to(|| HttpResponse::Ok().body("Ok")))
    // }).keep_alive(); // 使用"SO_KEEPALIVE" socket 选项

    let _three = HttpServer::new(||{
        App::new().route("/", web::get().to(|| HttpResponse::Ok().body("Ok")))
    }).keep_alive(None); // 关闭keep alive

    one.bind("127.0.0.1:8080")?.run().await
}

