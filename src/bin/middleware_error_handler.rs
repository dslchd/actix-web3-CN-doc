use actix_web::{HttpServer, App, web, HttpResponse, dev, Result, http};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};

/// 自己定义500错误响应
fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().wrap(
            ErrorHandlers::new()
                .handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500)
        ).service(web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("success")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
        )
    }).bind("127.0.0.1:8080")?
        .run().await
}
