use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/// ## Hello world 示例
///  * 1. Request 使用一个async 异步函数来处理，它接收0个或多个参数，这些参数能被Request提取，并且返回一个被
///  转换成HttpResponse类型的 trait.
///  * 2. 下面的异步处理函数，可以直接使用内置宏来附加路由信息。这允许你指定响应方法与资源path.
///  * 3. 另外你也可以不使用路由宏来注册handler函数，可以使用像v2版本的写法，例如下面的manual_hello函数.

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world Rust!")
}

#[get("/test")]
async fn test() -> String { "Direct Response String".to_string()}

#[get("/")]
async fn other() -> impl Responder {
    HttpResponse::Ok().body("Default Other Resp")
}

/// post echo server
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// 不使用声明式路由，手工构建响应fn 这种就是v2版本的写法
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("manual hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // main 入口函数
    HttpServer::new(|| {
        App::new()
            // v3 版本的写法
            .service(hello)
            .service(test)
            .service(echo)
            .service(other)
        // v2 版本的写法
            .route("/hey", web::get().to(manual_hello))
    }).bind("127.0.0.1:8080")?
        .run().await
}

