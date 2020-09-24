## HTTP2.0
如果有可能 `actix-we` 会自动的将链接升级成 _HTTP/2_ .

## Negotiation
_HTTP/2_ 协议是基于TLS的且需要 [TLS ALPN](https://tools.ietf.org/html/rfc7301).

当前仅有 `rust-openssl` 有支持.

`alpn` 需要协商启用该功能, 当启用时, `HttpServer` 提供了一个 [bind_openssl](https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_openssl)
的方法来操作.

toml文件中加入如下依赖:

```toml
actix-web = { version = "3", features = ["openssl"] }
openssl = { version = "0.10", features = ["v110"] }
```

```rust
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn index(_req: HttpRequest) -> impl Responder {
    "Hello."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 载入 ssl keys
    // 为测试创建自签名临时证书:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
```
不支持升级到RFC3.2节中描述的HTTP/2.0模式. _HTTP/2_ 明文链接与TLS链接都支持.

具体示例参考 [examples/tls](https://github.com/actix/examples/tree/master/rustls).