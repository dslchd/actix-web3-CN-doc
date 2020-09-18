## URL分发(URL Dispatch)
URL分发提供了一种使用简单模式匹配的方式去将URL映射到处理器函数代码上. 如果请求关联的路径信息被一个模式匹配, 那么一个特定的handler处理
函数对象会被激活.

一个请求处理器的功能是接收零个或多个能在请求中被提取的参数(ie, `impl FromRequest`) 且返回一个能被转换成HttpResponse的类型(ie, `impl HttpResponse`).
更多的信息参见[handler section](https://actix.rs/docs/handlers/).

## 资源配置(Resource configuration)
资源配置它是向应用中添加一个新资源的行为. 资源具有名称,该名称用于URL生成的标识符. 该名称也允许开发者向现有资源添加路由. 资源也有一种
模式, 这意味着可以匹配 _URL_ _PATH_ 中的一部分(比如格式与端口号后的一部分: /foo/bar in the URL http://localhost:8080/foo/bar?q=value).
它不匹配查询 _Query_ 的部分(?号后面的部分,比如: q=value in  http://localhost:8080/foo/bar?q=value)

使用 `App::route()` 方法提供了一个简单的方式注册路由. 这个方法添加单个路由到应用的路由表中去. 这个方法接收一个路径模式, HTTP方法和一个处理函数.
同一路径下`route()`方法可以被多次调用, 因此相同的资源路径可以注册多个路由.

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/user", web::post().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
虽然 _App::route()_ 提供了一个简单的方式来注册路由, 为了访问完整的资源配置,一个不同的方法可以使用. 它就是 `App::service()` 方法：添加
单个资源到应用路由表中去. 这个方法接收一个路径, guards, 和一个或多个routers.

```rust
use actix_web::{guard, web, App, HttpResponse};

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

pub fn main() {
    App::new()
        .service(web::resource("/prefix").to(index))
        .service(
            web::resource("/user/{name}")
                .name("user_detail")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get().to(|| HttpResponse::Ok()))
                .route(web::put().to(|| HttpResponse::Ok())),
        );
}
```
如果一个资源不包含任何的路由或者没有任何能匹配上的路由,它将返回 _NOT FOUND_ HTTP响应(也就是404).

## 配置一个路由(Configuration a Route)
资源包含一组路由的集合. 每一个路由都依次有一组 `guards` 和 一个处理函数. 使用 `Resource::route()`方法创建一个新的路由,它返回一个新路由
实例的引用. 默认的情况下路由不包含任何的 guards, 因此所有的请求都可以被默认的处理器(HttpNotFound)处理.

应用程序传入请求是基于资源注册与路由注册期间定义的路由标准. 资源按 `Resource::route()` 注册的路由顺序来匹配所有的请求. 

一个路由可以包含任意数量 _guards_ 但仅仅只能有一个处理器函数.

```rust
#[actix_web::main]
fn main() -> std::io::Result<()> {
App::new().service(
    web::resource("/path").route(
        web::route()
            .guard(guard::Get())
            .guard(guard::Header("content-type", "text/plain"))
            .to(|| HttpResponse::Ok()),
    ),
)
}
```
在上面这个示例中, 如果GET请求中的header包含指定的 _Content-type_ 为 _text/plain_ 且路径为 `/path` `HttpResponse::Ok()`才会被返回.

如果资源没有任何匹配的路由,那么会返回 _NOT FOUNT_ 响应.

`ResourceHanlder::route()` 返回一个 `Route` 对象. Route使用类似builder模式来配置. 提供如下的配置方法: 
* `Route::guard()` - 注册一个新的守卫, 每个路由都能注册多个guards.
* `Route::method()` - 注册一个方法级守卫, 每个路由都能注册多个guards.
* `Route::to()` - 为某个路由注册一个处理函数. 仅能注册一个处理器函数. 通常处理器函数在最后的配置操作时注册.
* `Route::to_async()` - 注册一个异步处理函数. 仅能注册一个处理器函数. 通常处理器函数在最后的配置操作时注册.

## 路由匹配(Route matching)
路由配置的主要目的是针对一个URL路径模式去匹配(或者匹配)请求中的`path`. `path`代表被请求URL中的一部分.

_actix-web_ 做到这一点是非常简单的. 当一个请求进入到系统时, 对系统中存在的每一个资源配置声明,actix会根据声明的模式去检查请求中的路径.
这种检查按照 `App::service()` 方法声明的路径顺序进行. 如果找不到资源, 默认的资源就会作为匹配资源来使用.

当一个路由配置被声明时, 它可能包含路由的保护参数. 与路由声明关联的所有路由保护对于在检查期间用于给定请求路由的配置,都必须为ture
(译者注:换句话说, 路由上配置的所有guard,进来的请求都必须满足才能通过). 如果在检查期间注册在路由上的任意一个保护(guard)参数配置返回了`false`
那么当前路由就会被跳过，然后继续通过有序的路由集合进行匹配.

如果有任意一个路由被匹配上了, 那么路由匹配的过程就会停止与此路由相关联的处理函数就会被激活.如果没有一个路由被匹配上,就会得到一个 _NOT FOUND_ 的响应返回.

## 资源模式语法(Resource pattern syntax)





