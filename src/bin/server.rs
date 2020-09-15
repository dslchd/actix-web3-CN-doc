use actix_web::{web, App, HttpResponse, HttpServer, rt::System};
use std::sync::mpsc;
use std::thread;

/// ## HttpServer
/// HttpServer 负责处理Http请求.
///
/// HttpServer 接收一个应用程序工厂作为一个参数,且应用程序工厂必须有 Sync + Send 边界.
/// 会在多线程章节解释这一点.
///
/// 使用 bind() 方法来绑定一个指定的Socket地址，它可以被多次调用. 使用bind_openssl()或者bind_rustls()方法绑定
/// ssl Socket地址. 使用HttpServer::run()方法来运行一个Http 服务.
///
/// run()方法返回一个server类型的实例, server中的方法可以被用来管理HTTP服务器.
/// * pause() - 暂停接收进来的链接.
/// * resume() - 继续接收进来的链接.
/// * stop() - 停止接收进来的链接，且停止所有有worker线程后退出.
/// 下面的例子展示了如果在单独的线程中启动HTTP服务.
///
/// ## 多线程
/// HttpServer 自动启动一些 Http Workers(工作线程), 它的默认值是系统cpu的逻辑个数. 这个值你可以通过 HttpServer::workers()
/// 方法来自定义并覆盖.
///
/// 一旦workers被创建，它们每个都接收一个单独的应用程序实例来处理请求.应用程序State不能在这些workers线程之间共享，且处理程序可以自由操作状态副本，而无需担心并发问题.
///
/// 应用程序State不需要Send或者Sync，但是应用程序工厂必须要是Send + Sync (因为它需要在不同的线程中共享与传递).
///
/// 为了在worker 线程之间共享State, 可以使用Arc. 引入共享与同步后，应该格外的小心, 在许多情况下由于锁定共享状态而无意中造成了"性能成本".
///
/// 在某些情况下，可以使用更加有效的锁策略来减少这种"情能成本",举个例子，可以使用读写锁(read/write locks)来代替排它锁(mutex)来实现互斥性,
/// 但是性能最高的情况下，还是不要使用任何锁。
///
/// 因为每一个worker线程是安顺序处理请求的，所以处理程序阻塞当前线程，会并停止处理新的请求.
/// ```rust
/// fn my_handler() -> impl Responder {
///     std::thread::sleep(Duration::from_secs(5)); // 糟糕的实践方式，这样会导致当前worker线程停止处理新的请求.并挂起当前线程
///     "response"
/// }
/// ```
/// 因此，任何长时间的或者非cpu绑定操作(比如:I/O,数据库操作等)，都应该使用future或异步方法来处理.
///
/// 异步处理程序由工作线程(worker)并发执行，因此不会阻塞当前线程的执行.例如下面的使用示例:
/// ```rust
/// fn my_handler() -> impl Responder {
///     tokio::time::delay_for(Duration::from_secs(5)).await; // 这种没问题，工作线程将继续处理其它请求.
/// }
/// ```
/// 上面说的这种限制同样也存在于提取器(extractor)中. 当一个handler函数在接收一个实现了 FromRequest 的参数时，并且这个实现
/// 如果阻塞了当前线程，那么worker线程也会在运行时阻塞.
///
/// 因此，在实现提取器时必须特别注意，在需要的时候要异步实现它们.
///
/// ## SSL
/// 有两种方式来实现ssl的server. 一个是rustls一个是openssl. 在Cargo.toml文件中加入如下依赖:
/// ``` rust
/// [dependencies]
/// actix-web = { version = "3", features = ["openssl"] }
/// openssl = {version = "0.10"}
/// ```
/// ```rust
/// #[get("/")]
/// async fn index(_req: HttpRequest) -> impl Responder {
///     "Welcome"
/// }
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     // 载入ssl key
///     // 为了测试可以创建自签名的证书
///     // 'openssl req -x509 -newkey ras:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost' '
///     let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
///     builder
///         .set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
///     builder.set_certificate_chain_file("cert.pem").unwrap();
///
///     HttpServer::new(||{
///         App::new().service(index)
///     }).bind_openssl("127.0.0.1:8080")?
///         .run().await
/// }
/// ```
/// **注意:** HTTP2.0需要[tls alpn](https://tools.ietf.org/html/rfc7301)支持,目前仅仅只有openssl有alpn支持.
/// 更多的示例可以参考[examples/openssl](https://github.com/actix/examples/blob/master/openssl)
///
/// 为了创建生成key.pem与cert.pem，可以使用如下示例命令. **其它需要修改的地方请填写自己的主题**
/// ```shell
/// openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
/// ```
/// 要删除密码，然后复制 nopass.pem到 key.pem
/// ```shell
/// openssl rsa -in key.pem -out nopass.pem
/// ```
#[actix_web::main]
async fn main() {
    // 声明一个 多生产者单消费者的channel
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let sys = System::new("http-server");
        let server = HttpServer::new(|| {
            App::new().service(
                web::scope("/app")
                    .route("/test", web::get().to(|| HttpResponse::Ok().body("Ok")))
            )
        }).workers(4)  // 自定义workers数量
            .bind("127.0.0.1:8080")?
            .shutdown_timeout(60)// 设置shutdown 时间为60秒
            .run();
        let _ = tx.send(server);
        println!("New Http Server Started op Port 8080");
        sys.run() // 会启动一个 event loop 服务直到 stop()方法被调用
    });
    let serv = rx.recv().unwrap();
    //暂停接收新的链接
    serv.pause().await;
    // 继续接收新的链接
    serv.resume().await;
    // 停止服务
    serv.stop(true).await;
    println!("Http Server has been Stopped");
}
