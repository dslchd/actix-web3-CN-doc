use actix_web::{HttpServer, App, Result, web, guard, error, HttpResponse};
use serde::Deserialize;
use actix_web::web::Json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // 单独配置json
        let json_config = web::JsonConfig::default().limit(4096) // 限制最大playload为 4kb
            .error_handler(|err, _req|{
                // 创建自定义错误响应
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new().service(
            web::scope("/json")
                .app_data(json_config) // 设置JsonConfig配置
                .guard(guard::Header("Content-Type", "application/json"))
                .route("/getInfo", web::get().to(get_info))
        )
    }).bind("127.0.0.1:8080")?
        .run().await
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

async fn get_info(info: Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

