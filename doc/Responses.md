## 响应(Response)
一种类 builder 模式被使用来构造一个 `HttpResponse` 实例.  `HttpResponse` 提供了几个返回 `HttpResponseBuilder` 实例的方法,
此实例实现了一系列方便的方法用来构建响应.

查看文档 [documentation](https://docs.rs/actix-web/3/actix_web/dev/struct.HttpResponseBuilder.html) 获取类型描述.

方法 `.body`, `.finish`, 和 `.json` 完成响应的创建并返回构建好的 _HttpResponse_ 实例. 如果在同一构建器实例上多次调用这些方法,
构建器将会发生 panic(恐慌).

```rust
use actix_web::HttpResponse;

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body("data")
}
```

## 内容编码(Content encoding)
Actix-web 能使用 [Compress middleware](https://docs.rs/actix-web/3/actix_web/middleware/struct.Compress.html) 自动的压缩payloads.
支持下面的编解码器:
* Brotli
* gzip
* Deflate
* Identity

```rust
use actix_web::{get, middleware, App, HttpResponse, HttpServer};

#[get("/")]
async fn index_br() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index_br)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
根据中间件 `middleware::BodyEncoding` trait 的编码参数来压缩响应的payloads. 默认情况下 `ContentEncoding::Auto` 被使用. 如果使用
`ContentEncoding` 那么就会根据请求头中的 `Accept-Encoding` 来决定压缩.

`ContentEncoding::Identity` 可用来禁用压缩. 如果选择了其它方式的压缩, 那么该编解码器就会强制执行压缩.

例如对单个处理函数启用 `Brotli` 可以使用 `ContentEncoding::Br`.

```rust
use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

#[get("/")]
async fn index_br() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br)
        .body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index_br)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

或者在整个应用中配置:
```rust
use actix_web::{http::ContentEncoding, dev::BodyEncoding, HttpResponse};

async fn index_br() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware, web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .route("/", web::get().to(index_br))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

我们通过设置content encoding 的值为 `Identity` 方式来禁用内容压缩.

```rust
use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        // 禁用压缩
        .encoding(ContentEncoding::Identity)
        .body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

当我们处理已经存在压缩的body数据时(比如,当处理资产时), 通过手动设置 _header_ 中 `content-encoding` 的值为 `Identity` 的方式来避免压缩已经压缩过的数据.
```rust
use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

static HELLO_WORLD: &[u8] = &[
    0x1f, 0x8b, 0x08, 0x00, 0xa2, 0x30, 0x10, 0x5c, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9, 0x57,
    0x28, 0xcf, 0x2f, 0xca, 0x49, 0xe1, 0x02, 0x00, 0x2d, 0x3b, 0x08, 0xaf, 0x0c, 0x00, 0x00, 0x00,
];

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .header("content-encoding", "gzip")
        .body(HELLO_WORLD)
}
```

也可以在应用级别设置默认的内容编码, 默认情况下 `ContentEncoding::Auto` 被使用. 这意味着内容压缩是可以交涉的
(译者注: 换句话说,默认情况下是根据请求中的 Accept-encoding 来指定压缩方式的).

```rust
use actix_web::{get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br)) // 这里就是设置应用级别的编码压缩方式.
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Json响应(Json Response)
`Json` 类型允许使用正确的Json格式数据进行响应. 只需要返回 `Json<T>` 类型, 其中`T`是序列化为Json格式的类型.
```rust
use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

#[get("/a/{name}")]
async fn index(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyObj {
        name: obj.name.to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```



