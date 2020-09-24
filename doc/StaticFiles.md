## 个体文件(Individual file)
可以使用自定义路径模式和 `NameFile`来提供静态文件. 为了匹配路径尾端, 我们可以使用 `[.*]` 正则表达式.

```rust
use actix_files::NameFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

## 目录(Directory)
提供来自指定目录或子目录的文件, 可以使用 `Files`. `Files` 必须使用 `App::service()` 方法来注册, 否而它不能被用在子目录处理上.

```rust
use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", ".").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

默认情况下子目录的文件清单是被禁用的. 尝试去加载目录列表将返回一个 _404 NOT FOUND_ 的响应. 为了启用文件清单功能, 要使用
[Files::show_files_listing()](https://docs.rs/actix-files/0.2/actix_files/struct.Files.html) 方法.

除了显示某个目录中文件的列表外, 还可以重定向到特定的index文件. 可以使用 [Files::index_file()](https://docs.rs/actix-files/0.2/actix_files/struct.Files.html#method.index_file) 来配置这个重定向.

## 配置(Configuration)
`NameFiles` 提供一系列的可选项:
* `set_content_disposition` - 函数用来将文件mime映射为 `Content-Disposition` 类型.
* `use_etag` - 指定是否计算ETag值并将其包含在headers中.
* `use_last_modified` - 指定是否将文件修改的时间戳添加在header的 `Last-Modified`中.

上面所有的方法都是可选的,并且提供了最佳的默认值,同时你也可以定制它们中的任何一个.
```rust
use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, App, Error, HttpRequest, HttpServer};

#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

该配置也可以用于目录:
```rust
use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            fs::Files::new("/static", ".")
                .show_files_listing()
                .use_last_modified(true),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```