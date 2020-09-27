## 异步选项(Async Options)
我们(actix-web)提供了几个使用异步数据库适配器的示例工程:
* SQLx: https://github.com/actix/examples/tree/master/sqlx_todo
* Postgres: https://github.com/actix/examples/tree/master/async_pg
* SQLite: https://github.com/actix/examples/tree/master/async_db

## Diesel
Diesel的当前版本(v1)还不支持异步操作, 因此使用 `web::block` 函数将数据库操作装载到 _Actix_ 运行时线程池中是非常重要的.

你可以创建与应用程序在数据库上执行的所有操作相对应的动作功能.
```rust
fn insert_new_user(db: &SqliteConnection, user: CreateUser) -> Result<User, Error> {
    use self::schema::users::dsl::*;

    // 创建插入模型
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        id: &uuid,
        name: &user.name,
    };

    // 正常 diesel 操作
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&self.0)
        .expect("Error inserting person");

    let mut items = users
        .filter(id.eq(&uuid))
        .load::<models::User>(&self.0)
        .expect("Error loading person");

    Ok(items.pop().unwrap())
}
```

现在你应该使用例如像 `r2d2` 这样的包来设置数据库链接池, 这将使你的应用程序有许多数据库链接可用. 这也意味着多个处理函数可以同时操作数据库,
并且还能够接收新的链接. 简单的来说链接池是应用程序状态. 在这种情况下最好不要使用状态包装结构体(struct), 因为在池中会共享访问.

```rust
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 创建链接池
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // 启动HTTP服务
    HttpServer::new(move || {
        App::new::data(pool.clone())
            .resource("/{name}", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

现在, 在请求处理器中使用 `Data<T>` 提取器来从应用程序State中得到pool并进而得到链接. 这提供了一个可以传入到 `web::block` 闭包中的数据库链接.
然后只需要使用必要的参数调用 action 函数, `.await` 结果即可.

如果你返回了一个实现了 `ResponseError` 的错误类型, 示例中在使用 `?` 号操作符前将一个错误映射成一个 `HttpResponse` 中时, 这个操作并不是必须的.

```rust
async fn index(req: web::Data<DbPool>, name: web::Path<(String)>) -> impl Responder {
    let name = name.into_inner();

    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || actions::insert_new_user(&conn, &user))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    Ok(HttpResponse::Ok().json(user))
}
```

完整的示例参考这里: https://github.com/actix/examples/tree/master/diesel