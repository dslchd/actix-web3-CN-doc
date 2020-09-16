use actix_web::{HttpServer, get, App, Result, web, guard};
use serde::Deserialize;
use actix_web::web::Json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/json")
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

