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