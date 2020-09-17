use actix_web::{HttpServer, App, web, Responder, get};
use std::sync::atomic::{AtomicUsize, Ordering};

use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = AppState{count: Arc::new(AtomicUsize::new(0))};

    HttpServer::new(move ||{
        App::new().data(data.clone())
            .service(show_count)
            .service(add_one)
    }).bind("127.0.0.1:8080")?
        .run().await
}

#[derive(Clone)]
struct AppState {
    // AtomicUsize: 一个可以安全的在多个线程是安全共享的整形
    count: Arc<AtomicUsize>,
}

#[get("/")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {

    format!("count: {}", data.count.load(Ordering::Relaxed))
}

#[get("/add")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {

    data.count.fetch_add(1, Ordering::Relaxed);

    format!("count: {}", data.count.load(Ordering::Relaxed))
}



