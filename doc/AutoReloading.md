## 自动重载开发服务(Auto-Reloading Development Server)

在开发的过程中，让**Cargo**在代码发生变化时自动编译是非常方便的. 可以使用 [cargo-watch](https://github.com/passcod/cargo-watch) 非常
容易的来完成这件事.

```shell
cargo watch -x 'run --bin app'
```

## 值得注意点(Historical Note)

此页面的旧版本建议使用`systemfd`和`listenfd`的组合,但这也有些缺陷，很难正确的整合，尤其是在更广泛的开发工作流中. 我们考虑使用 `cargo-watch`
可以非常方便的达到自动重载(auto-reloading)的目的.

