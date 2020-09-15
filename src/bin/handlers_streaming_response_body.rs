use actix_web::{HttpServer, App, get, HttpResponse, Error};
use futures::stream::once;
use futures::future::ok;

/// ## 流式响应Body (Streaming response body)
/// 响应也可以是异步的. 在下面的案例中, body 必须实现Steam trait(Stream<Item=Bytes, Error=Error>)
#[actix_web::main]
async fn main() -> std::io::Result<()> {

}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>);
}

