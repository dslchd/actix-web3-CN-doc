use actix_web::{HttpServer, App, get, error, Result, dev::HttpResponseBuilder, http::header,
                http::StatusCode, HttpResponse, middleware::Logger};
use derive_more::{Display, Error};
use log::debug;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置环境变量参数
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=debug"); // 这里需要将actix_web的日志级别设置为debug
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // warp方法 注册一个中间件
            .wrap(Logger::default()) // 添加默认的日志设置
            .service(index)
            .service(user_error)
    }).bind("127.0.0.1:8080")?
        .run().await
}

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bod request")]
    BadClientData,
    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    // 重写 error_response() 方法使用默认实现
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    // 重写 status_code
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT
        }
    }
}

#[get("/error")]
async fn index() -> Result<&'static str, MyError> {
    let err =MyError::BadClientData;
    debug!("{}", err);
    Err(err)
}

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    Validation {
        field: String
    }
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset = utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::Validation { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

#[get("userError")]
async fn user_error() -> Result<&'static str, UserError> {
    let error = UserError::Validation {field: "username".to_string()};
    debug!("{}", error);
    Err(error)
}
