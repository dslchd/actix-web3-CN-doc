## 类型安全的信息提取器(Type-safe information extractor)
`actix-web` 提供了一个灵活的类型安全的请求信息访问者，它被称为提取器(extractors)(实现了 `impl FromRequest`).
默认下actix-web提供了几种extractors的实现.

提取器可以被作为处理函数的参数访问. `actix-web` 每个处理函数(handler function)最多支持10个提取器. 它们作为参数的位置没有影响.
```rust
async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}
```

# 路径(Path)
Path提供了能够从请求路径中提取信息的能力. 你可以从path中反序列化成任何变量.

因此，注册一个/users/{user_id}/{friend}的路径, 你可以反序列化两个字段, user_id和 friend.
这些字段可以被提取到一个 `tuple` (元组)中去, 比如: Path<u32, String> 或者是任何实现了 `serde trait`包中的 `Deserialize`
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


也可以提取信息到一个指定的实现了`serde trait`反序列化的类型中去. 这种serde的使用方式与使用元组等效.


另外你也可以使用 `get` 或者 `query` 方法从请求path中通过名称提取参数值:
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
查询 `Query` 类型为请求参数提供了提取功能. 底层是使用了 `serde_urlencoded` 包功能.

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
`Json`提取器允许你将请求body中的信息反序列化到一个结构体中. 为了从请求body中提取类型的信息，类型 T 必须要实现 `serde` 的 `Deserialize trait`.

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
一些提取器提供了一种可以配置提取过程的方式. `Json` 提取器就使用 `JsonConfig` 类来配置. 为了配置提取器，可以将配置对象通过`resource`的`.data()`
方法传进去.如果是Json提取器，它将返回一个 `JsonConfig`. 另外你也可以配置json的最大有效负载(playload)和自定义错误处理功能.

下面的示例限制了`playload`的最大大小为4kb,且使用一个自定义的`error`处理.
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

## 表单(Form)
当前仅支持url编码的形式. 使用`url-encoded`的body能被提取成一个指定的类型.这个类型必须实现`serde`包的`Deserialize trait`.

`FormConfig` 允许你配置提取的过程.

参考如下示例:
```rust
use actix_web::{post, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// 使用serde提取表单数据
/// 仅当 content type 类型是  *x-www-form-urlencoded* 是 handler处理函数才会被调用
/// 且请求中的内容能够被反序列化到一个 "FormData" 结构体中去.
#[post("/")]
async fn index(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}
```

## 其它(Other)
`Actix-web` 也提供了一些其它的提取器(extractors):
* [Data](https://docs.rs/actix-web/3/actix_web/web/struct.Data.html) - 如果你需要访问应用程序状态的话.
* HttpRequest - 如果你需要访问请求, `HttpRequest`本身就是一个提取器，它返回Self.
* String - 你可以转换一个请求的`playload`成一个`String`. 可以参考文档中的[example](https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html#example-2)
* bytes::`Bytes` - 你楞以转换一个请求的`playload`到`Bytes`中. 可以参考文档中的[example](https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html#example-4)
* Playload - 可以访问请求中的playload. [example](https://docs.rs/actix-web/3/actix_web/web/struct.Payload.html)

## 应用程序状态提取器(Application state extractor)
可以使用 `web::Data` 提取器在handler函数中访问应用程序状态(state); 然而state仅能作为一个只读引用来访问.如果你需要以可变(mutable)的方式
来访问state, 则它必须被实现.

**Beware**, actix为应用程序状态和处理函数创建多个副本. 它为每一个线程创建一个副本.

下面是存储了处理的请求数的处理程序示例:

```rust
use actix_web::{web, Responder};
use std::cell::Cell;

#[derive(Clone)]
struct AppState {
    count: Cell<i32>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let data = AppState {
        count: Cell::new(0),
    };

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .route("/", web::to(show_count))
            .route("/add", web::to(add_one))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
虽然处理函数可以工作，`self.0` 的值会有所不同，具体取决于线程数量和每个线程处理的请求数.正解的实现是使用`Arc` 和 `ActomicUsize` .
(因为它们是线程安全的).

```rust
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    count: Arc<AtomicUsize>,
}

#[get("/")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.load(Ordering::Relaxed))
}

#[get("/add")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.count.fetch_add(1, Ordering::Relaxed);

    format!("count: {}", data.count.load(Ordering::Relaxed))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        count: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .service(show_count)
            .service(add_one)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
小心使用这些同步原语像`Mutex` 或 `Rwlock` . `actix-web` 框架是异步处理请求的. 如果阻塞了执行线程，那么所有的并发请求都将在处理函数
阻塞. 如果你需要在多线程中共享或更新状态, 考虑使用`tokio`的同步原语.

