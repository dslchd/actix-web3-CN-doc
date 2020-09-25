## Actix是相关包的生态系统(Actix is an ecosystem of crates)

&emsp;Actix 表示几种事务. 它基于Rust中功能强大的actor系统, 最开始是基于 `actix-web` 系统来构建的.这是你最可能使用的一点. `actix-web` 为你提供了功能强大且能快速开发的Web开发框架.

&emsp;我们一般称 `actix-web` 是一个小巧且实用的框架. 出于这些目的, 这是一个微框架. 如果你已经是一个Rust程序员,
那么你很快就能在这里找到家(译者注:有家的感觉?), 但即使你是由其它编程语言过来的,你也会发现 `actix-web` 上手非常容易.

&emsp;使用 `actix-web` 开发的应用将暴露一个本机可执行文件中包含的HTTP服务器. 同样,你也可以将它放在其它的HTTP服务
之后, 比如像nginx. 即使完全没有其它的HTTP服务器, `actix-web` 也足以提供 _HTTP/1_ 和 _HTTP/2_ 以及支持
TLS(HTTPS)服务. 这用来构建分发小型服务很有用.

&emsp;最重要的是: `actix-web` 是运行在Rust 1.42 及更高的stable release 版本之上.