use actix_web::{HttpServer, App, get, middleware, HttpResponse};

#[get("/br")]
async fn index_br() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

async fn main() -> std::io::Result<()> {

}