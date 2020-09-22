use actix_web::{HttpServer, App, HttpResponse, web, get, HttpRequest, http};
use serde::Deserialize;
use actix_web::guard::Guard;
use actix_web::dev::RequestHead;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=debug"); // 这里需要将actix_web的日志级别设置为debug
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .guard(ContentTypeHeader)
                // .guard(guard::Not(ContentTypeHeader))  // 这一句会反转guard 含义，表示所有带 Content-Type 的请求都不能过.
                .service(show_users)
                .service(user_detail)
                .service(get_matches)
                .service(get_username)
        ).service(external_resource)
            .external_resource("youtube", "https://youtube.com/watch/{video_id}")
    }).bind("127.0.0.1:8080")?
        .run().await
}


#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("show_users")
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32, )>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[get("/matcher/{v1}/{v2}")]
async fn get_matches(req: HttpRequest) -> String {
    // 直接根据替换表达式名获取一个值
    let v1:u8 = req.match_info().get("v1").unwrap().parse().unwrap();

    let v2: String = req.match_info().query("v2").parse().unwrap();

    // 还可以使用 元组的模式匹配
    let (v3, v4): (u8, String) = req.match_info().load().unwrap();

    format!("Values {}, {}, {}, {}", v1, v2, v3, v4)
}

#[derive(Debug, Deserialize)]
struct Info {
    username: String,
}

#[get("/{username}/index.html")]
async fn get_username(data: web::Path<Info>) -> String {
    format!("{}", data.username)
}

#[get("/external")]
async fn external_resource(req: HttpRequest) -> HttpResponse {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();

    assert_eq!(url.as_str(),"https://youtube.com/watch/oHg5SJYRHA0");

    // 手动修改一下header中的内容
    HttpResponse::Ok().header("Content-Type","text/plain").body(url.into_string())
}

struct ContentTypeHeader;

impl Guard for ContentTypeHeader {
    fn check(&self, request: &RequestHead) -> bool {
        request.headers().contains_key(http::header::CONTENT_TYPE)
    }
}



