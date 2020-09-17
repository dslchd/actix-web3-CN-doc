extern crate derive_more;
use actix_web::{HttpServer, App, get, error, Result};

use derive_more::{Display, Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    }).bind("127.0.0.1:8080")?
        .run().await
}


#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

/// error_response() 方法使用默认实现
impl error::ResponseError for MyError {}

#[get("/error")]
async fn index() -> Result<&'static str, MyError> {
    Err(MyError{name: "test"})
}


