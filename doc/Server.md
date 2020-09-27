## Http服务器(The HTTP Server)
[HttpServer](https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html) 负责HTTP请求的处理.

`HttpServer` 接收一个应用程序工厂作为一个参数,且应用程序工厂必须有 `Sync` + `Send` 边界. 会在多线程章节解释这一点.

使用 `bind()` 方法来绑定一个指定的Socket地址,它可以被多次调用. 使用 `bind_openssl()` 或者 `bind_rustls()` 方法绑定
ssl Socket地址. 使用 `HttpServer::run()` 方法来运行一个 Http 服务.

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

`run()` 方法返回一个 `server` 类型的实例, server中的方法可以被用来管理HTTP服务器.
* pause() - 暂停接收进来的链接.
* resume() - 继续接收进来的链接.
* stop() - 停止接收进来的链接,且停止所有worker线程后退出.

下面的例子展示了如何在单独的线程中启动HTTP服务.

```rust
use actix_web::{web, App, HttpResponse, HttpServer, rt::System};
use std::sync::mpsc;
use std::thread;

#[actix_web::main]
async fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8080")?
        .shutdown_timeout(60) // <- 设置关机超时时间为60s.
        .run();

        let _ = tx.send(srv);
        sys.run()
    });

    let srv = rx.recv().unwrap();

    // 暂停接收新的链接
    srv.pause().await;
    // 继续接收新的链接
    srv.resume().await;
    // 停止服务
    srv.stop(true).await;
}
```

## 多线程(Multi-threading)
`HttpServer` 自动启动一些 Http Workers(工作线程), 它的默认值是系统cpu的逻辑个数. 这个值你可以通过 `HttpServer::workers()` 方法来自定义并覆盖.

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .workers(4); // <- 启动4个worker线程
}
```

一旦workers被创建,它们每个都接收一个单独的应用程序实例来处理请求.应用程序State不能在这些workers线程之间共享,且处理程序可以自由操作状态副本,而无需担心并发问题.

应用程序状态不需要 `Send` 或者 `Sync` , 但是应用程序工厂必须要是 `Send` + `Sync` (因为它需要在不同的线程中共享与传递).

为了在worker线程之间共享State, 可以使用 `Arc`. 引入共享与同步后,应该格外的小心,在许多情况下由于锁定共享状态而无意中造成了"性能成本".

在某些情况下,可以使用更加有效的锁策略来减少这种 "性能成本", 举个例子,可以使用读写锁 [read/write locks](https://doc.rust-lang.org/std/sync/struct.RwLock.html) 
来代替排它锁 [mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) 来实现互斥性,但是性能最高的情况下,还是不要使用任何锁.

因为每一个worker线程是按顺序处理请求的,所以当处理程序阻塞当前线程时,会停止处理新的请求.

```rust
fn my_handler() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5)); // <--  糟糕的实践方式,这样会导致当前worker线程停止处理新的请求.并挂起当前线程
    "response"
}
```

因此,任何长时间的或者非cpu绑定操作(比如:I/O,数据库操作等),都应该使用future或异步方法来处理. 异步处理程序由工作线程(worker)并发执行,
因此不会阻塞当前线程的执行.

例如下面的使用示例:
```rust
fn my_handler() -> impl Responder {
    tokio::time::delay_for(Duration::from_secs(5)).await; // 这种没问题,工作线程将继续处理其它请求.
    "response"
}
```
上面说的这种限制同样也存在于提取器(extractor)中. 当一个handler函数在接收一个实现了 `FromRequest` 的参数时, 并且这个实现
如果阻塞了当前线程,那么worker线程也会在运行时阻塞. 因此,在实现提取器时必须特别注意,在需要的时候要异步实现它们.

## SSL
有两种方式来实现ssl的server. 一个是 `rustls` 一个是 `openssl` . 在Cargo.toml文件中加入如下依赖:
```toml
[dependencies]
actix-web = { version = "3", features = ["openssl"] }
openssl = { version="0.10" }
```

```rust
use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 导入 ssl keys
    // 为了测试创建自签名临时证书:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
```

**注意:** HTTP2.0需要 [tls alpn](https://tools.ietf.org/html/rfc7301) 来支持, 目前仅仅只有 `openssl` 支持 `alpn`.
更多的示例可以参考 [examples/openssl](https://github.com/actix/examples/blob/master/openssl) .

为了创建生成key.pem与cert.pem,可以使用如下示例命令. **其它需要修改的地方请填写自己的主题**
```shell
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
```
要删除密码,然后复制 nopass.pem到 key.pem
```shell
openssl rsa -in key.pem -out nopass.pem
```

## 链接保持(Keep-Alive)
Actix 可以在keep-alive 链接上等待请求.

_keep alive_ 链接行为被server设置定义.
* 75, Some(75), KeepAlive::Timeout(75) - 开启keep alive 保活时间.
* None or KeepAlive::Disable - 关闭keep alive设置.
* KeepAlive::Tcp(75) - 使用 tcp socket SO_KEEPALIVE 设置选项.

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let one = HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(75); // <- 设置keep alive时间为75秒.

    // let _two = HttpServer::new(|| {
    //     App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    // })
    // .keep_alive(); // <- Use `SO_KEEPALIVE` socket option.

    let _three = HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(None); // <- 禁用 keep alive

    one.bind("127.0.0.1:8080")?.run().await
}
```

如果上面的第一个选项被选择,那么 _keep alive_ 状态将会根据响应的 _connection-type_ 类型来计算. 默认的 `HttpResponse::connection_type` 没有被定义
这种情况下会根据http的版本来默认是否开启keep alive.

_keep alive_ 在 HTTP/1.0是默认 **关闭** 的, 在 _HTTP/1.1_ 和 _HTTP/2.0_ 是默认**开启**的.

链接类型可以使用 `HttpResponseBuilder::connection_type()` 方法来改变.

```rust
 use actix_web::{http, HttpRequest, HttpResponse};
 async fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().connection_type(http::ConnectionType::Close) // 关闭链接
    .force_close() // 这种写法与上面那种写法二选一
    .finish()
}
```

## 优雅关机(Graceful shutdown)
`HttpServer` 支持优雅关机. 在接收到停机信号后,worker线程有一定的时间来完成请求. 超过时间后的所有worker都会被强制drop掉.
默认的shutdown 超时时间设置为30秒. 你也可以使用 `HttpServer::shutdown_timeout()` 方法来改变这个时间.

你可以使用服务器地址向服务器发送停止消息,并指定是否要正常关机, server的 `start()` 方法可以返回一个地址.

`HttpServer` 处理几种OS信号. _ctrl-c_ 在所有操作系统上都适用(表示优雅关机), 也可以在其它类unix系统上使用如下命令:
* SIGINT - 强制关闭worker
* SIGTERM - 优雅关闭worker
* SIGQUIT - 强制关闭worker

另外也可以使用 `HttpServer::disable_signals()` 方法来禁用信号处理.