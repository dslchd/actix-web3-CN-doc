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
actix使用的模式匹配语言在匹配模式参数上是很简单的事.

在路由中使用模式可以使用斜杠开头. 如果模式不是以斜杠开头, 在匹配的时候会在前面加一个隐式的斜杠. 例如下面的模式是等效的:
```text
{foo}/bar/baz
```

和:

```text
/{foo}/bar/baz
```
上面变量替换的部分,{}标识符的形式指定, 这意味着 “直到下一个斜杠符前,可以接受任何的字符”, 并将其用作 `HttpRequest.match_info()`对象中的名称.

模式中的替换标记与正则表达式 `[^{}/]+` 相匹配.

匹配的信息是一个 `Params` 对象, 代表基于路由模式从URL中动态提取出来的部分. 它可以作为 `request.match_info` 信息来使用.
例如, 以下模式定义一个字段(foo)和两个标记(baz,and bar): 

```text
foo/{baz}/{bar}
```
上面的模式将匹配这些URL, 生成以下的匹配信息:

```text
foo/1/2         -> Params {'baz': '1', 'bar':'2'}
foo/abc/def     -> Params {'baz': 'abc', 'bar':'def'}
```
然而下面的模式不会被匹配:
```text
foo/1/2/         -> Not match(trailing slash) (不匹配最后面一个斜杠)
bar/abc/def      -> First segment literal mismatch (第一个字段就不匹配)
```
字面路径 _/foo/biz.html 将和上面的路由模式匹配,且匹配的结果j  `Params{'name': 'biz' }`. 但是字面路径 _/foo/biz_ 不会被匹配,
是因为它由{name}.html表示的字段末尾不包含文字 _.html_ (因为它仅仅包含biz. 而不是biz.html).

为了要捕获两个分段, 可以使用两个替换标志符:
```text
foo/{name}.{ext}
```
字面路径 _/foo/biz.html_ 将会被上面的路由模式匹配, 并且匹配的结果将是 _Params {'name':'biz', 'ext':'html'}_ . 这种情况被匹配是因为
字面部分 _.(period)_ 在两个替换标记 _{name}_ 与 _{ext}_ 之间.

替换标记也可以使用正则表达式, 该正则表达式被用于确定路径段是否与标记匹配. 为了指定替换标记仅仅只匹配正则表达式定义的一组特定字符,你必须使用
稍微有扩展的替换标记语法. 在大括号内,替换标记名后必须跟一个冒号, 然后才是正则表达式. 与替换标记 `[^/]+` 相关联的是默认正则表达式匹配一个或
多个非斜杠的字符. 比如, 替换标记 _{foo}_ 可以更详细的写为 _{foo: [^/]+}_ 这种. 你可以将其更改为任意的正则表达式来匹配任意字符序列.比如: 
_{foo: \d+}_ 可以匹配 foo 后面的任意数字.

分段必须至少包含一个字符，这样才能匹配段替换标记. 例如,对于URL _/abc/_ :
* /abc/{foo} 不会匹配.
* /{foo} 将会匹配.

**注意**: 在匹配前, 将对路径(path)进行URL取消引号并将其解码为有效的unicode字符串, 且表示匹配路径段的值也将被URL取消引号.

因此,对于下面的模式:
```text
foo/{bar}
```
当它匹配下面这种URL时:
```text
http://example.com/foo/La%20Pe%C3%B1a
```
匹配字典看起来像下面这样(值是URL解码过的):
```text
Params {'bar': 'La Pe\xf1a'}
```
路径段中的文件字字符串会被代表解码后的值提供给actix. 如果你不想在模式中使用URL解码的值, 例如, 而不是这样:
```text
/Foo%20Bar/{baz}
```
你将使用以下内容:
```text
/Foo Bar/{baz}
```
有可能得到一种“尾部匹配”. 因此你必须使用正则表达式.
```text
foo/{bar}/{tail:.*}
```
上面的模式将和这些URL匹配, 生成如下匹配的信息:
```text
foo/1/2/                -> Params: { 'bar': '1', 'tail':'2' }
foo/abc/def/a/b/c       -> Params: { 'bar':u'abc', 'tail':'def/a/b/c' }
```

## 范围路由(Scoping Routes)
范围的界定可以帮助你组织共享跟帖的根路径. 你可以潜逃范围(scopes).

假设你要组织 "Users" 的端点路径. 这样的路径可能包含:
* /users
* /users/show
* /users/show/{id}

这些路径作用域的布局如下:
```rust
#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok.body("Show Users")
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok.body(format!("User Detail: {}", path.into_inner().0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(
            web::scope("/users")
            .service(show_users)
            .service(user_detail)
        )
    }).bind("127.0.0.1:8080")?
    .run().await
}
```
范围路径可以包含路径变量段来作为资源(译者注: 像上面示例的{id}). 与未限制范围的路径一致.

你可以从 `HttpRequest::match_info()` 方法中得到路径变量段值. `Path extractor` 还能提取范围级别的变量段.

## 匹配信息(Match information)
所有表示路径匹配段的值都可以在 `HttpRequest::match_info()` 中获取. 可以使用 `Path::get()` 方法来获取特定的值.

```rust
use actix_web::{get, App, HttpRequest, HttpServer, Result};

#[get("/a/{v1}/{v2}/")]
async fn index(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();
    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
    Ok(format!("Values {} {} {} {}", v1, v2, v3, v4))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```
对于上面这个例子来说, 路径 '/a/1/2',分别对应v1和v2并被解析成"1"和"2".

可以从尾部路径参数创建一个 `PathBuf` . 返回的 `PathBuf` 按百分比解码. 如果一个分段等于 ".." 则跳过前一个段.

出于安全的目的, 如果一个分段满足下面任何一个条件, 则返回一个 `Err` , 表明已满足条件:
* 解码的分段以 `. (除了 ..), *` 任意一个开头的.
* 解码的分段以 `:`, `>`, `<` 任意一个结尾的.
* 解码的分段包含了任意的 `/`
* 在Windows上, 解码段包含任意的:  ‘'
* 百分比编码导致无效的UTF-8 

由于这些条件的存在, 根据请求路径参数解析出来的一个 `PathBuf` 可以安全的插入其中, 或用作没有其它检查的路径后缀.

```rust
use actix_web::{get, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

#[get("/a/{tail:.*}")]
async fn index(req: HttpRequest) -> Result<String> {
    let path: PathBuf = req.match_info().query("tail").parse().unwrap();
    Ok(format!("Path {:?}", path))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

## 路径信息提取器(Path information extractor)
Actix支持类型安全的路径信息提取功能. `Path` 提取信息, 目的地(转换后)的类型可以以几种不同的形式来定义. 一种简单的方式是一个 `tuple` 类型.
元组中的每一个元素必须对应模式路径中的一个元素. 比如: 你可以匹配一个路径模式 `/{id}/{username}` 为一个 `Path<(u32, String, String)>`
但是 `Path<(u32,String, String)>` 这种类型就会失败.

```rust
use actix_web::{get, web, App, HttpServer, Result};

#[get("/{username}/{id}/index.html")] // <- 定义路径参数
async fn index(info: web::Path<(String, u32)>) -> Result<String> {
    let info = info.into_inner();
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

你也可以提取路径模式信息到一个结构体中. 在这种情况下结构体必须实现 **serde's** `Deserialize` trait.
```rust
use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// 使用serde提取路径信息
#[get("/{username}/index.html")] // <- 定义路径参数
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

为请求查询参数 [Query](https://docs.rs/actix-web/3/actix_web/web/struct.Query.html) 提供了类似的功能.

## 生成资源URL(Generating resource URLs)

使用 [HttpRequest.url_for()](https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.url_for) 方法去生成基于
资源模式的URLs. 比如, 你配置了一个以"foo" 为名称的资源,并且模式是 “{a}/{b}/{c}”, 你可能会这样做:
```rust
use actix_web::{get, guard, http::header, HttpRequest, HttpResponse, Result};

#[get("/test/")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", &["1", "2", "3"])?; // <- 为"foo"资源生成url

    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/test/{a}/{b}/{c}")
                    .name("foo") // <- 设置资源名, 然后它可以被 'url_for'使用
                    .guard(guard::Get())
                    .to(|| HttpResponse::Ok()),
            )
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

这将返回类似 ` http://example.com/test/1/2/3` 的字符串(至少在当前协议与主机名下包含 http://example.com). `url_for()` 方法返回 `Url object` 
对象以变你可以修改这个url(添加查询参数,锚定等). `url_for()` 方法仅能在已经命名的资源时调用, 否则将返回错误.

## 外部资源(External resources)
有效的URL资源可以注册成外部资源.它们仅用于生成URL,而在请求时不考虑匹配.

```rust
use actix_web::{get, App, HttpRequest, HttpServer, Responder};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

    url.into_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .external_resource("youtube", "https://youtube.com/watch/{video_id}")
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```


## 路径正规化并重定向(Path normalization and redirection to slash-appended routes)

规范化意味着:
* 在路径上添加斜杠
* 用一个替换多个斜杠

这样的好处是处理器能够正确的解析路径(path)并返回. 如果全部启用, 标准化条件的顺序为 1) 合并, 2) 合并且追加 3). 如果路径至少满足这些条件中
的一个, 则它将重定向到新路径.
+
```rust
use actix_web::{middleware, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .route("/resource/", web::to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
上面示例中的 `//resource///` 将会被重定向为 `/resource/` . 但是不能依赖此机制来重定向 _POST_ 请求.

带有斜杠的 _NOT FOUND_ 的重定向会将原POST请求转换成GET请求, 从而丢失原始请求POST中的所有数据.

可以针对GET请求注册规范化的路径:

```rust
use actix_web::{get, http::Method, middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(index)
            .default_service(web::route().method(Method::GET))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 使用应用程序前缀来编写应用(Using an Application Prefix to Compose Applications)
`web::scope()` 方法允许你设计一个指定的应用程序范围. 此范围表示资源的前缀, 前缀能附加到资源配置添加的所有资源模式中. 它可以用在将一组
路由安装在与它包含的可被调用位置的不同地方, 同时还可以保持相同的资源名称(译者注: 很难理解, 你就当是 scope是一组路由资源的前缀就行了, 这样
做是好管理,资源路径清晰).

示例如下:
```rust
#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}


#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .service(show_users)
                .service(user_detail),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
在上面的示例中, _show_users_ 路由的有效模式为 _/users/show_ 而不是 _/show_ 那是因为应用程序作用域(scope)会在该模式之前. 然后, 仅仅
当URL路径为 _/users/show_ 时路由才会被匹配, 并组当 `HttpRequest.url_for()` 函数被调用时, 它也会生成相同路径的URL.

## 自定义路由防护(Custom route guard)
你可以将路由防护(guard)看作是一个接收请求对象引用并返回ture或者false的简单函数. 一般来说一个防护它是实现了 `guard` trait的任何对象. 
Actix 提供了多个谓词, 你可以查看 [functions section](https://docs.rs/actix-web/3/actix_web/guard/index.html#functions) API 文档.

下面是一个简单的检查一个请求中是否包含指定 header 的防护示例:
```rust
use actix_web::{dev::RequestHead, guard::Guard, http, HttpResponse};

struct ContentTypeHeader;

// 实现Guard 并重写了check函数
impl Guard for ContentTypeHeader {
    fn check(&self, req: &RequestHead) -> bool {
        req.headers().contains_key(http::header::CONTENT_TYPE)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/",
            web::route()
                .guard(ContentTypeHeader) // 添加一个防护
                .to(|| HttpResponse::Ok()),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
在上面的示例中, 仅当请求头中包含 _CONTENT-TYPE_ 时才会调用index 处理器.

防护(guard)不能够访问和修改请求对象, 但是它可以存一些额外的信息, 参考[request extensions](https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.extensions)

## 修改防护(guard)值(Modifying guard values)
你可以通过将任意谓词的值包装在 `Not` 谓词中来反转其含义. 比如, 如果你想为除了 "GET" 以外的所有方法返回 "METHOD NOT ALLOWED" 响应,
可以使用如下示例方式操作:
```rust
use actix_web::{guard, web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/",
            web::route()
                .guard(guard::Not(guard::Get()))
                .to(|| HttpResponse::MethodNotAllowed()),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
如果提供的guards能匹配 `Any` guard 表示接收一个防护清单
```
guard::Any(guard::Get()).or(guard::Post())  // 任意的Get Post都被接受
```
如果要使所有提供的 guards 能匹配, 可以使用 `All` guard.
```text
guard::All(guard::Get()).and(guard::Header("Content-Type","plain/text")) 
```
(译者注: 上面的表示所有的get且header中ContentType为 "plain/text" 的请求才能接受)

## 改变默认 **NOT FOUND** 响应(Changing the default NOT FOUND response)
如果路径模式不能在路由表中发现或者资源没有匹配的路由, 那么默认的资源就会被使用. 默认的响应是 _NOT FOUND_ . 可以使用 `App::default_service()`
方法来重写 _NOT FOUND_ 响应. 这个方法具有接受与 `App::service()` 方法的常规资源配置相同配置的功能.
```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```


