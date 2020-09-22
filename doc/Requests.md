## 内容编码(Content Encoding)
Actix-web 能自动的解压缩 payloads (载荷). 支持如下几个解码器:
* Brotli
* Chunked
* Compress
* Gzip
* Deflate
* Identity
* Trailers
* EncodingExt

如果请求头中包含一个 `Content-Encoding` 头, 请求的payload会根据header中的值来解压缩. 不支持多重解码器. 比如: `Content-Encoding:br, gzip` .

## Json请求(Json Request)
对于Json body 的反序列化有一些可选项.

第一种选择是使用 _Json extractor_ (Json 提取器). 首先, 你要定义一个处理器函数将 `Json<T>` 作为一个参数来接收, 然后, 在处理器函数上
使用 `.to()` 方法来注册. 使用 `serde_json::Value` 作为类型 `T` , 可以接受任意有效的 json对象. 

```rust
use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// 使用 serde 来提取Info 
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```
你也可以手动的将payload导入内存中然后反序列化它.

在下面的示例中, 我们将反序列化一个 _MyObj_ struct. 我们必须首先导入request body 然后再反序列化json到一个Object对象中去.

```rust
use actix_web::{error, post, web, App, Error, HttpResponse};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload是一个流式字节对象
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // 限制payload在内存中的最大 大小
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body导入后现在我们使用 serde-json 来反序列化它
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- 发送响应
}
```
两种选择的完整使用示例参考 [examples directory](https://github.com/actix/examples/tree/master/json/)

## 分块传输编码(Chunked transfer encoding)
Actix自动的解码 _chunked_ 编码. `web::Payload` 已经包含解码成字节流的方式. 如果请求负载(payload)是使用支持的编解码器(br, gzip, deflate)
其中之一来压缩, 那么会被作为字节流来解码.

## 多重body(Multipart body)
Actix-web 使用扩展包 `actix-multipart` 来支持多重流(stream).

完整的使用示例可以查看[examples directory](https://github.com/actix/examples/tree/master/multipart/)

# Urlencoded body
Actix-web 通过 `web::Form` 提取器为应用程序 _application/x-www-form-urlencoded_ 编码提供支持, 该提取器解析为反序列化实例. 这些实例必须从 serde 实现
`Deserialize` trait.

以下几种情况, _Urlencoded_ 未来可能解决错误.
* content type 不是 `applicaton/x-www-form-urlencoded`.
* transfer encoding 是 `chunked`.
* content-length 超过 256k.
* payload 终止错误.

```rust
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/")]
async fn index(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().body(format!("username: {}", form.username))
}
```

## 流式请求(Streaming request)
_HttpRequest_ 是一个字节流对象. 它能用来读取请求 body的负载(payload).

在下面的示例中, 我们读取并分块打印 request payload.

```rust
use actix_web::{get, web, Error, HttpResponse};
use futures::StreamExt;

#[get("/")]
async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    Ok(HttpResponse::Ok().finish())
}
```

