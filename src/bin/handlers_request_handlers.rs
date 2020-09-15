use actix_web::{web, HttpServer, App, HttpRequest, get};
/// ## Request Handlers
/// 一个请求处理器，它是一个异步函数，可以接收零个或多个参数，而这些参数被实现了(ie, impl FromRequest)的请求所提取，
/// 并且返回一个被转换成 HttpResponse或者其实现(ie, impl Responder)的类型.
///
/// 请求处理发生在两个阶段:
///
/// 首先处理对象被调用，并返回一个实现了 Responder trait的任何对象.然后在返回的对象上调用 respond_to()方法，将其
/// 自身转换成一个 HttpResponse 或者 Error .
///
/// 默认情况下 actix-web 为 &‘static str , String 等提供了 Responder的标准实现.
///
/// 完整的实现清单可以参考 [Responder documentation](https://docs.rs/actix-web/3/actix_web/trait.Responder.html#foreign-impls)
///
/// 有效的 handler示例:
///
/// ```rust
/// async fn index(_req: HttpRequest) -> &'static str {
///     "Hello World"
/// }
/// async fn index_two(_req: HttpRequest) -> String {
///     "Hello world".to_string()
/// }
/// ```
/// 你也可以改变返回的签名为 impl Responder 它在要返回复杂类型时比较好用.
/// ```rust
/// async fn index(_req: HttpRequest) -> impl Responder {
///     Bytes::from_static(b"Hello world")
/// }
/// async fn index(_req: HttpRequest) -> impl Responder {
///     // ...
/// }
/// ```
#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(index_two)
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run().await
}

async fn index(_req: HttpRequest) -> &'static str {
    "Hello World"
}

#[get("/two")]
async fn index_two(_req: HttpRequest) -> String {
    "Hello world".to_string()
}

