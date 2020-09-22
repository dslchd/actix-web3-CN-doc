use actix_web::{HttpServer, App, web, get, post, middleware, HttpResponse, http::ContentEncoding, Result};
use actix_web::dev::BodyEncoding;
use serde::{Deserialize, Serialize};

#[get("/default")]
async fn index_default() -> HttpResponse {
    HttpResponse::Ok()
        //.encoding(ContentEncoding::Identity) // 通过这种方式可以禁用内容压缩.
        .body("data")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //包装一个中间件
            .wrap(middleware::Compress::default()) // 使用默认压缩方式
            //.wrap(middleware::Compress::new(ContentEncoding::Br)) // 这种是全局指定响应的 编码方式，这样就不用在每一个handler函数中处理了.
            .service(index_default)
            .service(index_br)
            .service(index_json)
    }).bind("127.0.0.1:8080")?
        .run().await
}


#[get("/br")]
async fn index_br() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br) //通过 encoding() 方法显示指定响应的编码
        .body("data")
}

#[derive(Deserialize, Debug)]
struct MyJsonReq {
    name: String,
}

#[derive(Serialize)]
struct MyJsonResponse {
    result: String,
}

#[post("/json")]
async fn index_json(info: web::Json<MyJsonReq>) -> Result<HttpResponse> {
    // 打印一下info
    println!("request: {:?}", info);
    let name:String = info.into_inner().name;
    let resp = MyJsonResponse { result: name };
    Ok(HttpResponse::Ok().json(resp))
    // 注意使用Json提取器的时候 header中的 Content-Type 要为 application/json 这相当为handler 添加了个 guard
 }
