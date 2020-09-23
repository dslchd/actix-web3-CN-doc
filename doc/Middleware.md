## 中间件(Middleware)
Actix-web的中间件系统允许我们在请求/响应处理时添加一些其它的行为. 中间件可以(hook into)挂接到进入的请求处理过程中. 使我们能修改请求以及
暂停请求处理来及早的返回响应.

中间件也可以挂接(hook into)响应处理过程中.

一般来说, 中间件参与以下操作:
* 请求预处理.
* 响应后置处理.
* 修改应用程序状态.
* 访问扩展服务(redis, logging, sessions).

每个 `App` , `scope`, `Resource` 都能注册中件间, 并且它以注册顺序相反的顺序来执行. 通常, 中间件是一种实现了 [Server trait](https://docs.rs/actix-web/3/actix_web/dev/trait.Service.html) 
和 [Transform trait](https://docs.rs/actix-web/3/actix_web/dev/trait.Transform.html) 的类型. 在trait中的每一个方法都有其默认实现.
每一个方法能立即返回一个结果或者一个 _future_ 对象. 

下面演示创建一个简单的中件间:
```rust
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;

/// 在中间件处理过程器有两步.
/// 1. 中间件初始化, 下一个服务链中作为一个参数中间件工厂被调用.
/// 2. 中间件的调用方法被正常的请求调用.
pub struct SayHi;

///中间件工厂是来自 actix_service 包下的一个 `Transform` trait.
/// `S` - 下一个服务类型
/// `B` - 响应body类型
impl<S, B> Transform<S> for SayHi
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SayHiMiddleware { service })
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service for SayHiMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
```

另外对于简单的使用, 你可以使用 [wrap_fn](https://docs.rs/actix-web/3/actix_web/struct.App.html#method.wrap_fn) 来创建一个小的临时的中间件:

```rust
use actix_service::Service;
use actix_web::{web, App};
use futures::future::FutureExt;

#[actix_web::main]
async fn main() {
    let app = App::new()
        .wrap_fn(|req, srv| {
            println!("Hi from start. You requested: {}", req.path());
            srv.call(req).map(|res| {
                println!("Hi from response");
                res
            })
        })
        .route(
            "/index.html",
            web::get().to(|| async {
                "Hello, middleware!"
            }),
        );
}
```

Actix-web提供了一些有用的中间件, 比如, `logging`, `user sessions`, `Compress` 等等.

## 日志(Logging)
日志被作为一种中间件来实现. 常见的做法是将日志中间件注册为应用程序第一个中间件. 必须为每个应用程序注册日志中间件.

`Logger` 中间件使用标准的日志包去记录日志信息. 为了查看日志你必须为 _actix_web_ 包启用日志功能. ([env_logger](https://docs.rs/env_logger/*/env_logger/) 或者其它的).

## 用法(Usage)
创建指定格式的 `Logger` 中件间. 可以使用默认 `default` 方法创建默认的 `Logger`, 它使用默认的格式:

```text
  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
```

```rust
use actix_web::middleware::Logger;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

下面是默认情况下日志的记录格式:
```text
INFO:actix_web::middleware::logger: 127.0.0.1:59934 [02/Dec/2017:00:21:43 -0800] "GET / HTTP/1.1" 302 0 "-" "curl/7.54.0" 0.000397
INFO:actix_web::middleware::logger: 127.0.0.1:59947 [02/Dec/2017:00:22:40 -0800] "GET /index.html HTTP/1.1" 200 0 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:57.0) Gecko/20100101 Firefox/57.0" 0.000646
```

## 格式(Format)
* %% - %号标识
* %a - 远程IP地址(如果使用了反向代码,则为代理的IP)
* %t - 当请求开始处理的时间
* %P - 请求子服务进程ID号
* %r - 请求的第一行
* %s - 响应状态码
* %b - 包含http响应头的响应字节大小
* %T - 服务请求所用的时间, 以秒为单位, 以.06f为浮点数格式
* %D - 服务请求所有时间, 以毫秒为单位
* %{FOO}i - 请求header中的["FOO"] (译者注: 这相当于可以在日志中取到自定义请求头中的内容)
* %{FOO}o - 响应header中的["FOO"] (译者注: 这相当于可以在日志中取到自定义响应头中的内容)
* %{FOO} - 系统环境中["FOO"]

## 默认头(Default headers)
为了设置默认响应头, `DefaultHeaders` 中间件可以被使用. 如果响应中已经包含了指定的响应头,则 _DefaultHeaders_ 中间件不会再次设置头.

```rust
use actix_web::{http, middleware, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .service(
                web::resource("/test")
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(
                        web::method(http::Method::HEAD)
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 用户会话(User sessions)
Actix-web提供了会话管理的通用解决方案.  [actix-session](https://docs.rs/actix-session/0.3.0/actix_session/) 中间件可以使用多种
后端类型来存储 session 数据.

默认情况下, 仅cookie会话后端被实现. 可以添加其它后端的实现.

[CookieSession](https://docs.rs/actix-session/0.3.0/actix_session/struct.CookieSession.html) 使用cookie作为会话存储.
`CookieSessionBackend` 创建的 session 仅限制用来存少于4000字节的数据, 因为payload必须适合单个cookie. 如果session包含了超过4000
字节的数据, 就会产生一个内部服务错误.

cookie可能要有签名的或者私有的安全策略. 每种都有各自的 `CookieSession` 构建函数.

一个签名的cookie可以被客户端查看,但不能被修改.一个私有的cookie客户端既不能修改也不可查看.

构造函数以key来作为参数. 这是Cookie会话的私钥,如果更改此值,所有的会话都会丢失.

一般来说你创建一个 `SessionStorage` 中间件并使用指定后端实现来初始化它, 比如一个 `CookieSession`. 为了访问session的数据必须使用
`Session` 提取器. 该方法返回一个 [Session](https://docs.rs/actix-session/0.3.0/actix_session/struct.Session.html) 对象,
并允许我们访问或设置session data.

```rust
use actix_session::{CookieSession, Session};
use actix_web::{web, App, Error, HttpResponse, HttpServer};

async fn index(session: Session) -> Result<HttpResponse, Error> {
    // 访问 Session数据
    if let Some(count) = session.get::<i32>("counter")? {
        session.set("counter", count + 1)?;
    } else {
        session.set("counter", 1)?;
    }

    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}!",
        session.get::<i32>("counter")?.unwrap()
    )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                CookieSession::signed(&[0; 32]) // <- 基于Session中间件创建 cookie
                    .secure(false),
            )
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 错误处理

