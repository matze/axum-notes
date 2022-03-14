# Database access with axum-sqlx-tx

[axum-sqlx-tx][] is an alternative, layer-based approach to using SQLx directly.
One big advantage is that transactions are committed or rolled back
automatically if any of the inner requests succeed or return an error.


## Dependencies

```toml
{{#include Cargo.toml:6:}}
```


## Code

Unlike before, we do not have to manage the SQL pool in an `std::Arc<>`
ourselves, hence it is enough to just create the pool

```rust,noplayground
{{#include src/main.rs:10:29:}}
```

and add the layer to the routes:

```rust,noplayground
{{#include src/main.rs:65:79:}}
```

Because the `Tx` type implements sqlx' executor interface, all we need to change
is the type of the parameter:

```rust,noplayground
{{#include src/main.rs:37:63:}}
```


## Run

Start the server with

```
cargo run --bin with-sqlx-tx
```

and add posts with

```
curl -X POST -H 'Content-Type: application/json' -d '{"title": "Hello", "content": "World"}' http://127.0.0.1:3000/api/posts
```


[axum-sqlx-tx]: https://docs.rs/axum-sqlx-tx/latest/axum_sqlx_tx/

