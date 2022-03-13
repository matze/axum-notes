# Database access with SQLx

[SQLx][] is an async-first SQL crate to access various SQL databases. While
compile-time verification of queries is one of the outstanding features, we will
use ordinary queries here to show how to use it together with axum.


## Dependencies

```toml
{{#include Cargo.toml:6:}}
```


## Code

In this example, we will implement a super simple blog backend, consisting of
posts with a title and some content string. We accept two routes `/api/post` and
`/api/posts/`, the latter used to query all existing posts and to add a new one.

As usual, import some used modules first:

```rust,noplayground
{{#include src/main.rs:0:8:}}
```

Now, we define a small helper struct that keeps the connection pool alive as
well as an implementation to set up the (in-memory SQLite) database:

```rust,noplayground
{{#include src/main.rs:10:36:}}
```

Let's define a struct to represent a post and add the handlers to insert and
query posts:

```rust,noplayground
{{#include src/main.rs:38:70:}}
```

As you can see we can re-use the same struct (and benefit from type-safety
guarantees) for both serialization and deserialization in the database
as well as for sending and receiving posts to and from the client by deriving
the appropriate traits.

All that's left is setting up the server itself:

```rust,noplayground
{{#include src/main.rs:72:88:}}
```


## Run

Start the server with

```
cargo run --bin with-sqlx
```

and add posts with

```
curl -X POST -H 'Content-Type: application/json' -d '{"title": "Hello", "content": "World"}' http://127.0.0.1:3000/api/posts
```


[SQLx]: https://docs.rs/crate/sqlx/latest
