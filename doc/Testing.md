## 测试(Testing)
每一个应用程序都应该经过良好的测试. Actix-web 提供了执行单元与集成测试的工具.

## 单元测试(Unit Testing)
为了单元测试, actix_web提供了一个请求的builder类型. `TestRequest` 实现了类 builders 模式. 你可以使用　`to_http_requests()` 方法来生成
一个　`HttpRequest` 实例, 并且你可以用它来调用处理函数.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}
```

## 集成测试(Integration tests)
有几种测试应用程序的方法. Actix-web可用于在真实的HTTP服务器中使用特定的处理程序来运行程序. `TestRequest::get()`, `TestRequest::post()`,
可以被用作发送请求到测试服务器.

为了创建一个测试的 `Service`, 可以使用 `test::init_service` 方法来接受一个常规的 `App` 构建器.

更多信息查看 [API Documentation] (https://docs.rs/actix-web/3/actix_web/test/index.html).

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_index_post() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }
}
```

如果你需要更复杂的应用程序配置, 则测试应该与普通的应用程序创建非常类似. 比如说, 你可能需要初始化应用程序状态(State). 使用 `data` 方法
创建一个 `App` 并且附加状态, 这就像使用普通应用程序一样.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(
            App::new()
                .data(AppState { count: 4 })
                .route("/", web::get().to(index)),
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: AppState = test::read_response_json(&mut app, req).await;

        assert_eq!(resp.count, 4);
    }
}
```

## 流式响应测试(Stream response tests)
如果你需要测试流(Stream)的生成, 只要调用 `take_body()` 方法并转换结果 `ResponseBody` 到一个 _future_ 中并执行它, 比如当你在测试
[Server Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events) 时.

```rust
use std::task::Poll;
use bytes::Bytes;
use futures::stream::poll_fn;

use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::{web, http, App, Error, HttpRequest, HttpResponse};

async fn sse(_req: HttpRequest) -> HttpResponse {
    let mut counter: usize = 5;

    // yields `data: N` where N in [5; 1]
    let server_events = poll_fn(move |_cx| -> Poll<Option<Result<Bytes, Error>>> {
        if counter == 0 {
            return Poll::Ready(None);
        }
        let payload = format!("data: {}\n\n", counter);
        counter -= 1;
        Poll::Ready(Some(Ok(Bytes::from(payload))))
    });

    HttpResponse::build(StatusCode::OK)
        .set_header(http::header::CONTENT_TYPE, "text/event-stream")
        .set_header(
            http::header::CONTENT_ENCODING,
            ContentEncoding::Identity.as_str(),
        )
        .streaming(server_events)
}

pub fn main() {
    App::new().route("/", web::get().to(sse));
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures_util::stream::StreamExt;
    use futures_util::stream::TryStreamExt;

    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_stream() {
        let mut app = test::init_service(App::new().route("/", web::get().to(sse))).await;
        let req = test::TestRequest::get().to_request();

        let mut resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        // first chunk
        let (bytes, mut resp) = resp.take_body().into_future().await;
        assert_eq!(bytes.unwrap().unwrap(), Bytes::from_static(b"data: 5\n\n"));

        // second chunk
        let (bytes, mut resp) = resp.take_body().into_future().await;
        assert_eq!(bytes.unwrap().unwrap(), Bytes::from_static(b"data: 4\n\n"));

        // remaining part
        let bytes = test::load_stream(resp.take_body().into_stream()).await;
        assert_eq!(bytes.unwrap(), Bytes::from_static(b"data: 3\n\ndata: 2\n\ndata: 1\n\n"));
    }
}
```