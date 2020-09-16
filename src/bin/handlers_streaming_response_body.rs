use actix_web::{HttpServer, App, get, HttpResponse, Error};
use futures::stream::once;
use futures::future::ok;
use bytes::Bytes;

/// ## 流式响应Body (Streaming response body)
/// 响应也可以是异步的. 在下面的案例中, body 必须实现Steam trait(Stream<Item=Bytes, Error=Error>)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(stream)
    }).bind("127.0.0.1:8080")?
        .run().await
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));

    HttpResponse::Ok().content_type("application/json").streaming(body)
}

