use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpServer, App, web};
use actix_service::{Service, Transform};

use futures::future::{ok, Ready};
use futures::{Future, FutureExt};

use std::pin::Pin;
use std::task::{Context, Poll};


/// 中间件使用示例所表达的意图是:
/// 在请求进来时且并处理函数处理之前，我们可以对请求做一些操作。
/// 在响应返回前，我们可以对响应做一些操作.
/// 这种方式给了用户更多可扩展，可定制化的可能.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().wrap_fn(|req, srv| {
           println!("Hi form start. You requested: {}", req.path());
            srv.call(req).map(|res| {
                println!("Hi form response");
                res
            })
        });
        app.route("/middleware", web::get().to(|| async {
            "Hello Middleware"
        }))
    }).bind("127.0.0.1:8080")?
        .run().await
}

/// 在中间件处理过程器有两步.
/// 1. 中间件初始化, 下一个服务链中作为一个参数中间件工厂被调用.
/// 2. 中间件的调用方法被正常的请求调用.
pub struct SayHi;

///中间件工厂是来自 actix_service 包下的一个 `Transform` trait.
/// `S` - 下一个服务类型
/// `B` - 响应body类型
impl<S, B> Transform<S> for SayHi
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SayHiMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SayHiMiddleware { service })
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service for SayHiMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}

