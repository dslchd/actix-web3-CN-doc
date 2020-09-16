## 类型安全的信息提取器(Type-safe information extractor)
actix-web 提供了一个灵活的类型安全的请求信息访问者，它被称为提取器(extractors)(实现了 impl FromRequest).
默认下actix-web提供了几种extractors的实现.

提取器可以被作为处理函数的参数访问. actix-web 每个处理函数(handler function)最多支持10个提取器. 它们作为参数的位置没有影响.
```rust
async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}
```

# 路径(Path)
Path提供了能够从请求路径中提取信息的能力. 你可以从path中反序列化成任何变量.

因此，注册一个/users/{user_id}/{friend}的路径, 你可以反序列化两个字段, user_id和 friend.
这些字段可以被提取到一个 tuple(元组)中去, 比如: Path<u32, String> 或者是任何实现了 serde trait包中的 Deserialize
的结构体(structure)

请参见下面的示例:
```rust
use actix_web::{get, web, Result};

/// 从 path url "/users/{user_id}/{friend}" 中提取参数
/// {user_id} - 反序列化为一个 u32
/// {friend} - 反序列化为一个 String
#[get("/users/{user_id}/{friend}")] // <- 定义路径参数
async fn index(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
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


也可以提取信息到一个指定的实现了serde trait反序列化的类型中去. 这种serde的使用方式与使用元组等效.


另外你也可以使用 get 或者 query 方法从请求path中通过名称提取参数值:
```rust
#[get("/users/{userid}/{friend}")] // <- 定义路径参数
async fn index(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("userid").parse().unwrap();

    Ok(format!("Welcome {}, userid {}!", name, userid))
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

## 查询(Query)
查询 Query 类型为请求参数提供了提取功能. 底层是使用了 serde_urlencoded 包功能.

```rust
use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// 仅仅在请求查询参数中有 'username' 字段时才会被调用
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}
```

## Json
Json提取器允许你将请求body中的信息反序列化到一个结构体中. 为了从请求body中提取类型的信息，类型 T 必须要实现 serde 的 Deserialize trait.

参考如下示例:
```rust
use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// 将request body中的信息反序列化到 Info 结构体中去
#[get("/")]
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}
```
一些提取器提供了一种可以配置提取过程的方式. Json 提取器就使用 JsonConfig 类来配置. 为了配置提取器，可以将配置对象通过resource的.data()
方法传进去.如果是Json提取器，它将返回一个 JsonConfig. 另外你也可以配置json的最大有效负载(playload)和自定义错误处理功能.

下面的示例限制了playload的最大大小为4kb,且使用一个自定义的error处理.
```rust
use actix_web::{error, web, App, FromRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// 反序列化request body中的信息到 Info结构体中，且设置 playload大小为 4kb
async fn index(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // 配置json提取器
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // 创建一个自定义的错误类型
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new().service(
            web::resource("/")
                // 通过.app_data()改变json 提取器配置
                .app_data(json_config)
                .route(web::post().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

