use actix_web::{HttpServer, App, web, get, HttpRequest};
use serde::Deserialize;

/// ## 类型安全的信息提取器
/// actix-web 提供了一个灵活的类型安全的请求信息访问者，它被称为提取器(extractors)(实现了 impl FromRequest).
/// 默认下actix-web提供了几种extractors的实现.
///
/// 提取器可以被作为处理函数的参数访问. actix-web 每个处理函数(handler function)最多支持10个提取器. 它们作为参数的位置没有影响.
///
///## 路径(Path)
/// Path提供了能够从请求路径中提取信息的能力. 你可以从path中反序列化成任何变量.
///
/// 因此，注册一个/users/{user_id}/{friend}的路径, 你可以反序列化两个字段, user_id和 friend.
/// 这些字段可以被提取到一个 tuple(元组)中去, 比如: Path<u32, String> 或者是任何实现了 serde trait包中的 Deserialize
/// 的结构体(structure)
///
/// 也可以提取信息到一个指定的实现了serde trait反序列化的类型中去. 这种serde的使用方式与使用元组等效.
///
/// 另外你也可以使用 get 或者 query 方法从请求path中通过名称提取参数值
///
/// 请参见下面的示例:
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // 注册一个/users/{user_id}/{friend} 的路由path
        // user_id 被反序列化为一个u32
        // friend 被反序列化为一个String
        // {} 占位符
        App::new().route("/users/{user_id}/{friend}", web::get().to(get_user))
            .service(get_obj)
            .service(query)
    }).bind("127.0.0.1:8080")?
        .run().await
}

/// 反序列化成一个元组
async fn get_user(web::Path((user_id, friend)): web::Path<(u32, String)>) -> String {
    format!("Welcome {}, user_id {}!", friend, user_id)
}

#[get("/getObj/{user_id}/{friend}")]
async fn get_obj(info: web::Path<User>) -> String {
    // 创建一个myInfo
    let my_info = User::new(18, "dsl".to_string());
    // 获取请求参数中的user信息
    println!("req user:{:?}", info);
    // 判断id是否相等
    if my_info.user_id == info.user_id {
        "Good! Equal user_id".to_string()
    } else {
        // 否则返回一个新的String
        format!("this is new User [user_id:{}, friend:{}]", my_info.user_id, my_info.friend)
    }
}

#[get("/query/{age}/{username}")] // 定义请求路径参数
async fn query(req: HttpRequest) -> String {
    let age: u32 = req.match_info().get("age").unwrap().parse().unwrap();
    let username:String = req.match_info().query("username").parse().unwrap();
    format!("Hello {} your age:{}", username, age)
}

#[derive(Deserialize, Debug)]
struct User {
    user_id: u32,
    friend: String,
}

impl User {
    // create MyInfo
    fn new(user_id: u32, friend: String) -> Self {
        User { user_id, friend }
    }
}
