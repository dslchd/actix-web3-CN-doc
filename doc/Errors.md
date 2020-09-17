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

```

