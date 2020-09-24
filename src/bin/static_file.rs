use actix_web::{HttpServer, App, get, HttpRequest, Result};
use actix_files::NamedFile;
use std::path::PathBuf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(get_file_by_name)
            // 使用.service()方法注册一个目录，并调用show_files_listing方法列出所有文件清单
            // show_files_listing() 返回的是一个 html格式 的response 且 response header中 content-type: text/html,
            .service(actix_files::Files::new("/getDir", "D://testDir").show_files_listing())
    }).bind("127.0.0.1:8080")?
        .run().await
}


/// 通过一个指定的文件名获取一个文件
/// filename: 必须是一个文件的绝对路径比如在 windows上 D://a.txt
#[get("/getFile/{filename:.*}")] // 使用正则表达式 .* 表示任意扩展名的文件
async fn get_file_by_name(req: HttpRequest) -> Result<NamedFile> {
    // 得到一个PathBuf 它是一个mut 的path
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();

    let file = NamedFile::open(path)?;
    Ok(file) // 返回文件的内容
}

