## 编写一个应用程序(Writing an Application)
`actix-web` 里面提供了一系列可以使用rust来构建web server的原语。它提供了路由，中间件，request预处理，response的后置处理等。

所有的 `actix-web` 服务都围绕App实例来构建. 它被用来注册路由资源和中间件. 它也存储同一个scope内所有处理程序之间共享的应用程序状态.

应用的 `scope` 扮演所有路由命名空间的角色, 比如, 为所有的路由指定一个应用级范围(scope), 那么就会有一个相同前缀的url路径.
应用前缀总是包含一个 "/" 开头，如果提供的前缀没有包含斜杠，那么会默认自动的插入一个斜杠.

比如应用使用 `/app` 来限定, 那么任何使用了路径为 `/app`, `/app/` 或者 `/app/test` 的请求都将被匹配，但是` /application` 这种path不会被匹配.

```rust
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // 所有资源与路由加上前缀...
            web::scope("/app")
                // ...因此handler的请求是对应 `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

在此示例中，将创建具有 `/app` 前缀和 `index.html` 资源的应用程序.因此完整的资源路径url就是 `/app/index.html`.

更多的信息,将会在 `URL Dispatch` 章节讨论.

## 状态(State)
应用程序状态(State)被同一作用域(Scope)内的所有路由和资源共享.State 能被 `web::Data<T>` 来访问，其中 `T` 是 state的类型. State也能被中间件访问.

让我们编写一个简单的应用程序并将应用程序名称存储在状态中,你可以在应用程序中注册多个State.

```rust
use actix_web::{get, web, App, HttpServer};

// 这个结构体代表一个State
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- 得到app名

    format!("Hello {}!", app_name) // <- 响应app名
}
```

并且在初始化 app 时传递状态(state),然后再启动应用程序:

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
你可以在应用中注册任意数量的状态类型.

## 共享可变状态(Shared Mutable State)
`HttpServer` 接收一个应用程序工厂而不是一个应用程序实例, 一个 `HttpServer` 为每一个线程构造一个应用程序实例.
因此必须多次构造应用程序数据,如果你想在两个不同的线程之间共享数据,一个可以共享的对象可以使用比如: ``Sync + Send`.

内部 `web::Data` 使用 `Arc`. 因此为了避免创建两个 `Arc`， 我们应该在在使用 `App::app_data()` 之前创建好我们的数据.

下面的例子中展示了应用中使用可变共享状态, 首先我们定义state并创建处理器(handler).

```rust
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex 必须安全的在线程之间可变
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- 得到 counter's MutexGuard
    *counter += 1; // <- 在 MutexGuard 内访问 counter

    format!("Request number: {}", counter) // <- 响应counter值
}
```

并在 `App` 中注册数据:

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // 移动 counter 进闭包中
        App::new()
            // 注意这里使用 .app_data() 代替 data
            .app_data(counter.clone()) // <- 注册创建的data
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 使用一个应用级Scope去组合应用(Using an Application Scope to Compose Applications)
`web::scope()` 方法允许你设置一个资源组前缀. 它表示所有资源类型(或者说是一组资源定位符)前缀配置. 

比如说:

```rust
#[actix_web::main]
async fn main() {
    let scope = web::scope("/users").service(show_users);
    App::new().service(scope);
}
```

在上面的示例中, `show_users` 路由将是具有 `/users/show` 而不是 `/show` 路径的有效路由模式, 因为应用程序的scope参数被添加到该模式之前.
所以只有在URL路径为 `/users/show` 时, 路由才会匹配, 当使用路由名称 `show_users` 调用 `HttpRequest.url_for()` 函数时, 它也会生成同样的
URL路径.

## 应用防护和虚拟主机(Application guards and virtual hosting)
其实"防护"(guards)可以是说是actix-web为handler函数提供的一种安全配置.

你可以将防护看成一个接收请求对象引用并返回ture或者false的简单函数. 可以说guard是实现了Guard trait的任何对象. `actix-web` 提供了几种开箱即用的 `guards`.
你可以查看 [functions section](https://docs.rs/actix-web/3/actix_web/guard/index.html#functions)API文档.

其中一个guards就是 `Header`. 它可以被用在请求头信息的过滤.
```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("user"))),
            )
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 可配置(Configure)
为了简单与可重用, `App` 与 `web::Scope` 两者都提供了 `configure` 方法. 此功能让配置的各个部分在不同的模块甚至不同的库(library)
中移动时非常有用. 比如说, 一些资源的配置可以被移动到不同的模块中.

(译者注: 其实这是一种拆分管理，一般来说可以提高代码重用，减少修改某个Scope组时可能带来的影响其它模块的错误.)

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

// 此功能可以位于其他模块中
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

// 此功能可以位于其他模块中
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
上面例子的结果是:
```text
/         -> "/"
/app      -> "app"
/api/test -> "test"
```

每一个 `ServiceConfig` 都有它自己的 `data`, `routers`, 和 `services`.