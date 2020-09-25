## 请求处理器(Request Handlers)
一个请求处理器,它是一个异步函数,可以接收零个或多个参数,而这些参数被实现了(比如, [impl FromRequest](https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html) )的请求所提取,
并且返回一个被转换成HttpResponse或者其实现(比如, [impl Responder](https://docs.rs/actix-web/3/actix_web/trait.Responder.html) )的类型.

请求处理发生在两个阶段.首先处理对象被调用,并返回一个实现了 `Responder` trait的任何对象.然后在返回的对象上调用 `respond_to()` 方法,将其
自身转换成一个 `HttpResponse` 或者 `Error` .

默认情况下 _actix-web_ 为 `&‘static str` , `String` 等提供了 `Responder` 的标准实现.

完整的实现清单可以参考 [Responder documentation](https://docs.rs/actix-web/3/actix_web/trait.Responder.html#foreign-impls) .

有效的 handler示例:

```rust
async fn index(_req: HttpRequest) -> &'static str {
    "Hello World"
}
async fn index_two(_req: HttpRequest) -> String {
    "Hello world".to_string()
}
```

你也可以改变返回的签名为 `impl Responder` 它在要返回复杂类型时比较好用.

```rust
async fn index(_req: HttpRequest) -> impl Responder {
    Bytes::from_static(b"Hello world")
}
```

```rust
async fn index(_req: HttpRequest) -> impl Responder {
    // ...
}
```

## 返回自定义类型(Response with custom Type)
为了想直接从处理函数返回自定义类型的话, 需要这个类型实现 `Responder` trait.

让我们创建一个自定义响应类型，它可以序列化为一个 `application/json` 响应.

```rust
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use futures::future::{ready, Ready};

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // 创建响应并设置Content-Type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn index() -> impl Responder {
    MyObj { name: "user" }
}
```

## 流式响应body(Streaming response body)
响应主体也可以异步生成. 在下面的案例中, body 必须实现 Steam trait `Stream<Item=Bytes, Error=Error>` , 比如像下面这样:

```rust
use actix_web::{get, App, Error, HttpResponse, HttpServer};
use bytes::Bytes;
use futures::future::ok;
use futures::stream::once;

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(stream))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

## 不同的返回类型(两种) (Different Return Types(Either))
有时候你需要在响应中返回两种不同的类型, 例如，你可以进行错误检查并返回错误,返回异步响应或需要两种不同类型的任何结果.

对于这种情况, 你可以使用 `Either` 类型. `Either` 允许你组合两个不同类型的responder到一个单个类型中去.

请看如下示例:

```rust
use actix_web::{Either, Error, HttpResponse};

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

async fn index() -> RegisterResult {
    if is_a_variant() {
        // <- 变体 A
        Either::A(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // <- 变体 B
        Either::B(Ok("Hello!"))
    }
}
```
(译者注: 相当于将不同分支的不同类型包装到某一个类型中再返回).