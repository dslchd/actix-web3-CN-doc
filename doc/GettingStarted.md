## 安装Rust(Installing Rust)
如果你还没有安装Rust, 我们建议你使用 `rustup` 来管理你的Rust安装. [official rust guide](https://doc.rust-lang.org/book/ch01-01-installation.html) 
提供了精彩的安装起步指导.

Actix Web 当前支持的Rust最低版本(MSRV) 是1.42. 执行 `rustup update` 命令Rust可用的最佳版本. 因此本指南假设你在 Rust 1.42或更高的版本上运行.

## Hello, world!
基于Cargo 命令创建一个新的字节码工程, 并转换到新的目录:

```shell script
cargo new hello-world
cd hello-world
```
在你的工程下的 `Cargo.toml` 文件内添加 `Actix-web` 的依赖.
```toml
[depandencies]
actix-web = "3"
```

使用异步函数的请求处理器接受零个或多个参数. 这些参数能从请求中提取(查看 `FromRequest` trait) 且返回一个能被转换成一个 `HttpResponse`
(查看 `Responder` trait) 的类型.

```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
```

注意,其中某些处理器使用内置宏直接附加了路由信息. 这就允许你指定处理器程序响应的方法与路径. 你也将在下面的示例中看到如何不使用路由宏来注册
一个其它的路由.

下一步,创建一个 `App` 实例并注册请求处理器. 为使用了路由宏的处理器来使用 `App::service` 方法设置(路由), 并且 `App::route` 可以手动的
注册路由处理器,并声明一个路径(path)和方法(译者注: 比如post或者get). 最后应用程序使用 `HttpServer` 来启动, 它将使你的 `App` 作为一个
"application factory" 来处理传入的请求.

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

使用 `Cargo run` 来编译并运行程序. 使用actix运行时的 `#[actix_web::main]` 宏用来执行异步main函数. 现在你可以访问 `http://localhost:8080/` 
或者任何一个你定义的路由来查看结果.
