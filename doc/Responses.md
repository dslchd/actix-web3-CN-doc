## 响应(Response)
一种类 builder 模式被使用来构造一个 `HttpResponse` 实例.  `HttpResponse` 提供了几个返回 `HttpResponseBuilder` 实例的方法,
此实例实现了一系列方便的方法用来构建响应.

查看文档 [documentation](https://docs.rs/actix-web/3/actix_web/dev/struct.HttpResponseBuilder.html) 获取类型描述.

方法 `.body`, `.finish`, 和 `.json` 完成响应的创建并返回构建好的 _HttpResponse_ 实例. 如果在同一构建器实例上多次调用这些方法,
构建器将后 panic(恐慌).

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


