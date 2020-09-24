## Websockets
Actix-web使用 `actix-web-actors` 包来对WebSockets提供支持. 可以通过 [web::Payload](https://docs.rs/actix-web/3/actix_web/web/struct.Payload.html) 将请求的有效payload转换成 
[ws::Message](https://docs.rs/actix-web-actors/2/actix_web_actors/ws/enum.Message.html) 流, 然后使用组合器处理实时消息.
但是使用 _http actor_ 处理WebSocket通信更加简单.

下面是一个简单的 WebSocket 回声(echo) 示例:
```rust
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

/// 定义 HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// ws 消息处理器
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

完整可用的 WebSocket echo 服务示例请查看 [examples directory](https://github.com/actix/examples/tree/master/websocket/)

[websocket-chat directory](https://github.com/actix/examples/tree/master/websocket-chat/) 这里面提供了一个聊天服务器示例,
可以通过websocket或TCP链接来实现聊天功能.