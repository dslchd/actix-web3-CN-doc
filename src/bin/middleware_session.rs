use actix_web::{HttpServer, App, HttpResponse, Error, get};
use actix_session::{Session, CookieSession};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().wrap(CookieSession::signed(&[0;32]) // 基于 Session 中间件创建一个cookie
            .secure(false)
        ).service(index)
    }).bind("127.0.0.1:8080")?
        .run().await
}

#[get("/cookie")]
async fn index(session: Session) -> Result<HttpResponse, Error> {
    // 访问  session 数据
    if let Some(count) = session.get::<i32>("counter")? {
        session.set("counter", count + 1)?;
    } else {
        session.set("counter", 1)?;
    }

    Ok(HttpResponse::Ok().body(
        format!("Counter is : {}",
            session.get::<i32>("counter")?.unwrap() // get::<i32> 类型必须声明
    )))
}
