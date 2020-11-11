# actix-web 3.0 中文文档

## 1.说明
基本上算是翻译了官文档,但是示例并不一定和官方的相同. 所有的示例代码都源自官方文档中的示例,但又不完全与之相同.

算是一边学习一边理解写出来的demo代码且全部都能正常运行.

可以使用如下命令 + 指定文件名执行并查看结果:

```shell script
cargo run --bin hello_world
```

**另外:** `Actix-Web` 的网络部分是基于[Tokio](https://tokio.rs/tokio/tutorial) 来实现的. 因此要想更加深入的了解`Actix-web`的实现细节, `Tokio`是你
必须要学习和了解的框架. `Tokio` 的中文文档指南请参考: [这里](https://github.com/dslchd/tokio-cn-doc).

## 2.文档索引
### 介绍(Introduction)
[欢迎(Welcome)](doc/WelcomeToActix.md)

[什么是Actix(What is Actix)](doc/WhatIsActix.md)
### 基础(Basics)
[起步(Getting Started)](doc/GettingStarted.md)

[应用(Application)](doc/Application.md)

[服务器(Server)](doc/Server.md)

[处理器(Handlers)](doc/Handlers.md)

[提取器(Extractors)](doc/Extractors.md)

### 高级(Advanced)
[错误(Errors)](doc/Errors.md)

[URL分发(URL Dispatch)](doc/URLDispatch.md)

[请求(Requests)](doc/Requests.md)

[响应(Responses)](doc/Responses.md)

[测试(Testing)](doc/Testing.md)

[中间件(Middleware)](doc/Middleware.md)

[静态文件(Static Files)](doc/StaticFiles.md)

### 协议(Protocols)
[Websockets](doc/Webscokets.md)

[HTTP/2](doc/HTTP2.md)

## 模式(Patterns)
[数据库(Databases)](doc/Databases.md)

## 图解(Diagrams)
[HTTP服务初始化(HTTP Server Initialization)](doc/HTTPServerInitialization.md)

[链接生命周期(Connection Lifecycle)](doc/ConnectionLifecycle.md)

## API文档
[actix](https://docs.rs/actix)

[actix-web](https://docs.rs/actix-web/)

## 3.其它
由于水平有限,在翻译过程中过程中难免有错误或遗漏,可以发现后及时向我提出(提 issue).

希望此文档能给不想看英文原文或英文不太好的朋友, 在使用或学习 `Actix-web` 与 `Rust` 来开发Web应用时带来帮助,
大家共同提高, 为Rust的流行作出丁点贡献.

**如果觉得给你的学习带来了帮助, 可以帮忙点个star, 这将是我一直同步更新下去的动力.**