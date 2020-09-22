use actix_web::{HttpServer, App, post, web, Error, HttpResponse, error};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use serde_json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index_manual)
    }).bind("127.0.0.1:8080")?
        .run().await
}


#[derive(Deserialize, Serialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZ: usize = 262144; // 256k 最大playload

/// 手动反序列化json 到一个 Object中去
#[post("/manual")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload 是一个字节流
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // 限制内存中 payload 最大大小
        if (body.len() + chunk.len()) > MAX_SIZ {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body 被导入了，现在我们使用 serde_json 反序列化它
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj))  // 返回响应
}

