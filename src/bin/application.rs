use actix_web::{web, get, App, HttpServer, Responder, guard, HttpResponse};
use std::sync::Mutex;

/// ## 写一个应用
/// * actix-web 里面提供了一系列可以使用rust来构建web server的原语。它提供了路由，中间件，request预处理，response的后置处理等。
/// * 所有的actix-web servers都围绕App实例来构建. 它被用来注册路由资源来中间件. 它也存储同一个scope内所有处理程序之间共享的应用程序状态.
/// * 应用前缀总是包含一个 "/" 开头，如果提供的前缀没有包含斜杠，那么会默认自动的插入一个斜杠.
/// * 比如应用使用 /app 来限定，那么任何使用了路径为 /app, /app/ 或者 /app/test的请求都将被匹配，但是 /application这种path不会被匹配.
/// * 下面使用async main 函数来创建一个app 实例并注册请求处理器.
/// * 使用App::service 来处理使用路由宏，或者你也可以使用App::route来手功注册路由处理函数，声明一个path与方法.
/// * 最后使用HttpServer来启动服务，并处理incoming请求.
/// * 使用cargo run 运行，然后访问http://localhost:8080/ 或其它路由path 就可以看到结果.
/// * 下面这个例子使用 /app 前缀开头且以一个 index.html作用资源路径，因此完整的资源路径url就是 /app/index.html.
/// * 更多的信息，将会在URL Dispatch章节。
///
/// ## State
/// * 应用程序状态(State)被同一作用域(Scope)内的所有路由和资源共享。
/// * State 能被web::Data<T> 来访问，其中 T是 state的类型. State也能被中间件访问.
///
/// 让我们编写一个简单的应用程序并将应用程序名称存储在状态中,你可以在应用程序中注册多个State
///
/// ## 共享可变State
/// HttpServer接收一个应用程序工厂而不是一个应用程序实例,一个HttpServer 为每一个线程构造一个应用程序实例.
///
/// 因此必须多次构造应用程序数据,如果你想在两个不同的线程之间共享数据，一个可以共享的对象应用使用比如: Sync + Send
///
/// 内部 web::Data 使用 Arc. 因此为了避免创建两个 Arc， 我们应该在在使用App::app_data() 之前创建 好我们的数据。
/// 下面的例子中展示了应用中使用可变共享状态，首先我们定义state并创建处理器(handler).
///
/// ## 使用一个应用级Scope去组合应用
/// web::scope()方法允许你设置一个资源组前缀. 它表示所有资源类型(或者说是一组资源定位符)前缀配置。
/// 下面的 /app 就是这种使用方式，可以方便管理一组资源.
///
/// ## 应用防护和虚拟主机
/// 其实"防护"(guards)可以是说是actix-web为handler函数提供的一种安全配置.
/// 你可以将防护看成一个接收请求对象引用并返回ture或者false的简单函数. 可以说guard可以是实现了Guard trait的任何对象.
///
/// actix-web 提供了几种开箱即用的guards. 你可以在api文档中查找.
/// 其中一个guards就是 Header. 它可以被用在请求头信息的过滤.
///
/// ## 可配置
/// 为了简单与可重用，App与web::Scope两者都提供了configure方法. 此功能让配置的各个部分在不同的模块甚至不同的库(library)
/// 中移动时非常有用.
///
/// 其实这是一种拆分管理，一般来说可以提高代码重用，减少修改某个Scope组时可能带来的影响其它模块的错误.
/// 每一个ServiceConfig 都有它自己的 data, routers, 和 services

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 外部声明一个counter
    let counter = web::Data::new(AppStateWithCounter{counter:Mutex::new(0)});
    HttpServer::new(move ||{
        // 移动所有权
        App::new()
            // 在初始化的时候添加一个状态，并启动应用, 也就是说，这里设置的data,可以被同一Scope中的所有route共享到
            .data(AppState{app_name: String::from("Actix-web 3.0 demo")})
            // 设置一个可变的State 在多个线程中共享, 适合在多个线程中需要修改的场景
            .app_data(counter.clone())// 注册counter,为什么要用clone? 因为它需要在每个线程中共享
            .service(get_state)
            .configure(config) // 配置
            .configure(second_config)
            .service(
            // 所有以 /app 开头的path都将被匹配
            web::scope("/app")
                // 为 /app 资源组添加一个Header guard Http Header 的Content-Type 必须为指定的类型
                .guard(guard::Header("Content-Type","application/html"))
                // 这里会处理 /app/index.html的 get 请求
                .route("/index.html", web::get().to(index))
                // 同一个scope下再注册一个route
                .route("/getAppInfo", web::get().to(app_info))

        )
            .route("/", web::get().to(mutable_counter))
    }).bind("127.0.0.1:8080")?
        .run().await
}

async fn index() -> impl Responder {
    "hello actix-web 3.0"
}
async fn app_info() -> String {
    "This is app Info".to_string()
}

// 这个struct代表state
struct AppState {
    app_name: String,
}

#[get("/state/getState")]
async fn get_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name) // 返回app name
}

// 可变共享计数器，可以在多个线程之间共享的state
struct AppStateWithCounter {
    counter: Mutex<i32>, // Mutex 排它锁，可以安全的在多个线程之间操作
}

async fn mutable_counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // lock 会阻塞当前线程，直到它可用为止
    *counter += 1; // 解引用访问counter中的值，并 + 1
    format!("Request number : {}", counter) // 返回
}

/// 第一种配置function
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/t")
            .route(web::get().to(|| HttpResponse::Ok().body("This is oneConfig Response")))
    );
}

/// 第二种配置function
fn second_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/secondScope")
            .guard(guard::Header("Content-Type", "application/text"))
            .route("/test",web::get().to(|| HttpResponse::Ok().body("This is Second Config Response")))
    );
}