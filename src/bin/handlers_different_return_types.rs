use actix_web::{HttpServer, App, get, HttpResponse, Either, Error};
use rand::Rng;


/// ## 不同的返回类型(两种) Different Return Types(Either)
/// 有时候你需要在响应中返回两中不同的类型, 例如，您可以进行错误检查并返回错误，返回异步响应或需要两种不同类型的任何结果。
///
/// 对于这种情况, 你可以使用 Either类型, Either允许你组合两个不同类型的responder到一个单个类型中去.
///
/// 请看如下示例
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(index)
    }).bind("127.0.0.1:8080")?
        .run().await
}

type RegisterResult = Either<HttpResponse, Result<String, Error>>;

#[get("/")]
async fn index() -> RegisterResult {
    // 产生一个 0-9的随机整数
    let rand_num = rand::thread_rng().gen_range(0,9);
    if rand_num < 5 {
        Either::A(HttpResponse::Ok().body("number less then 5"))
    }else {
        let res = format!("Great! This number is {}", rand_num);
        Either::B(Ok(res))
    }
}



