use actix_web::{HttpServer, web , Responder, App};
use std::cell::Cell;

/// 非线程安全版本的 应用程序状态使用示例
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化data
    let data = AppState {count: Cell::new(0)};
    HttpServer::new(move ||{
        App::new().data(data.clone())
            .route("/", web::get().to(show_count))
            .route("/add", web::get().to(add_one))
    }).bind("127.0.0.1:8080")?
        .run().await
}

#[derive(Clone)]
struct AppState {
    // Cell 可用在内部可变场景 内部提供get/set 来修改
    count : Cell<i32>
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}
