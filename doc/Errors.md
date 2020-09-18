## 错误(Errors)
Actix-web使用它自己的`actix_web::error::Error`类型和`actix_web::error:ResponseError` trait 来处理web处理函数中的错误.

如果一个处理函数在一个`Result`中返回`Error`(指普通的Rust trait `std::error:Error`),此`Result`也实现了`ResponseError` trait的话,
actix-web将使用相应的`actix_web::http::StatusCode` 响应码作为一个Http response响应渲染.默认情况下会生成内部服务错误:
```rust
pub trait ResponseError {
    fn error_response(&self) -> Response<Body>;
    fn status_code(&self) -> StatusCode;
}
```
`Response` 将兼容的 `Result` 强制转换到Http响应中:
```rust
impl<T: Responder, E: Into<Error>> Responder for Result<T, E>{}
```
上面代码中的`Error`是actix-web中的error定义, 并且任何实现了`ResponseError`的错误都会被自动转换.

Actix-web提供了一些常见的非actix error的 `ResponseError` 实现. 例如一个处理函数返回一个 `io::Error`,那么这个错误将会被自动的转换成一个
`HttpInternalServerError`:
```rust
use std::io;
use actix_files::NamedFile;

fn index(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html"))
}
```
参见完整的 `ResponseError` 外部实现[the actix-web API documentation](https://docs.rs/actix-web/3/actix_web/error/trait.ResponseError.html#foreign-impls)

## 自定义错误响应示例(An example of a custom error response)
这里有一个实现了`ResponseError`的示例, 使用`derive_more`声明错误枚举.

```rust
use actix_web::{error, Result};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
// 为 `error_response()` 方法使用默认实现
impl error::ResponseError for MyError {}

async fn index() -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}
```
当上面的 `index` 处理函数执行时, `ResponseError` 的一个默认实现 `error_response()` 会渲染500(服务器内部错误)返回.

重写 `error_response()` 方法来产生更多有用的结果:

```rust
use actix_web::{
    dev::HttpResponseBuilder, error, get, http::header, http::StatusCode, App, HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/")]
async fn index() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}
```

## 错误帮助(Error helpers)
Actix-web提供了一个错误帮助功能的集合, 这个集合在从其它错误生成指定的HTTP错误码时非常有用.下面我们转换 `MyError`, 它没有实现
`ResponseError` trait, 使用`map_err`来返回400(bad request):

```rust
use actix_web::{error, get, App, HttpServer, Result};

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

#[get("/")]
async fn index() -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "test error" });

    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}
```
查看[The documentation for actix-web's `error` `module` ](https://docs.rs/actix-web/3/actix_web/error/struct.Error.html)
了解完整的错误帮助清单.

## 错误日志(Error logging)
Actix 所有错误日志都是 `WARN` 级别的. 如果应用日志级别启用 `DEBUG` 和 `RUST_BACKTRACE`, 回溯日志也会被启用.这些可以使用环境变量进行配置:
```text
RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo run
```
使用 error backtrace 的`Error`类型如果可用. 如果底层的异常(failure)(译者注: 这里翻译成异常好点) 不提供回溯(backtrace)，则构造一个新的回溯，指向发生转换的点（而不是错误的根源).

## 错误处理的最佳实践(Recommended practices in error handling).
考虑将应用产生的错误分成两个大类是非常有用的: 这意味着一部分面向用户，另外一部分则不是.

前者的一个示例是, 我们可以使用失败来批定个 `UserError` 的枚举, 该枚举封装了一个 `ValidationError` , 以便在用户输入错误时返回: 

```rust
use actix_web::{
    dev::HttpResponseBuilder, error, get, http::header, http::StatusCode, App, HttpResponse,
    HttpServer,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
```
这将完全按预期的方式运行, 因为编写使用 `display` 定义的错误消,其目的是为了用户要明确读取.

然而, 并不是所有的错误都要返回 - 在服务端环境捕获的许多异常我们都想让它对用户是隐藏的(不展示给用户看). 例如, 数据库关闭了导致的客户端
连接超时, 或 HTML 模板渲染时的格式错误. 在这些情况下最好将错误映射为适合用户使用的一般错误.

下面的示例，将内部错误映射为一个面向用户的 `InternalError`.

```rust
use actix_web::{
    dev::HttpResponseBuilder, error, get, http::header, http::StatusCode, App, HttpResponse,
    HttpServer,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/")]
async fn index() -> Result<&'static str, UserError> {
    do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}
```
通过将错误划分为面向用户的与非面向用户的部分, 我们可以确保我们不会意外的将应用程序内部错误暴露给用户，因为这部分错误并不是用户想看到的.

## 错误日志(Error Logging)
使用 `middleware::Logger` 示例:
```rust
use actix_web::{error, get, middleware::Logger, App, HttpServer, Result};
use log::debug;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<&'static str, MyError> {
    let err = MyError { name: "test error" };
    debug!("{}", err);
    Err(err)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

