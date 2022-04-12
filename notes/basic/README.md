## Handling Ctrl-C gracefully

axum re-exports hyper's [Server][] which provides the [with_graceful_shutdown][]
method that is easily combined with tokio's [signal::ctrl_c][] function
(available under the `signal` feature flag):

```rust, noplayground
axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .with_graceful_shutdown(async {
        tokio::signal::ctrl_c().await.expect("failed to listen to ctrl-c");
    })
    .await?;
```

[Server]: https://docs.rs/hyper/latest/hyper/server/struct.Server.html
[with_graceful_shutdown]: https://docs.rs/hyper/latest/hyper/server/struct.Server.html#method.with_graceful_shutdown
[signal::ctrl_c]: https://docs.rs/tokio/latest/tokio/signal/fn.ctrl_c.html
