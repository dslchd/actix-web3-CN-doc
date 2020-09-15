use actix_web::{HttpServer, App, HttpResponse, Responder, HttpRequest, Error,get};
use serde::Serialize;
use futures::future::{ready, Ready};


/// ## Response with custom Type (返回自定义类型)
/// 为了直接从处理函数返回自定义类型的话, 需要这个类型实现 Responder trait.
///
/// 让我们创建一个自定义响应类型，它可以序列化为一个 application/json 响应.
/// 先在Cargo.toml文件中添加如下依赖项:
///
/// ```rust
/// serde = "1.0.116"
/// futures = "0.3.5"
/// serde_json = "1.0.57"
/// ```
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(index)
    })
        .bind(SERVER_ADDRESS)?
        .run().await
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str
}

//响应Content-Type
const CONTENT_TYPE:&str = "application/json";
const SERVER_ADDRESS:&str = "127.0.0.1:8080";

/// 自定义Responder实现
impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;


    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // 先把self 序列化成一个json字符串
        let body = serde_json::to_string(&self).unwrap();

        // 创建响应并设置Content-Type
        ready(Ok(HttpResponse::Ok().content_type(CONTENT_TYPE).body(body)))
    }
}

#[get("/")]
async fn index() -> MyObj {
    MyObj{name: "user"}
}

