use actix_web::{HttpServer, App, HttpResponse, middleware::Logger, web, middleware};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志 info 级别
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new().wrap(Logger::default())
            // 设置日志格式
            .wrap(Logger::new("%a %{User-Agent}i"))
            // 包装一个中间件 设置默认响应header
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .route("/logging", web::get().to(|| HttpResponse::Ok().body("Hello logging")))
    }).bind("127.0.0.1:8080")?
        .run().await
}

