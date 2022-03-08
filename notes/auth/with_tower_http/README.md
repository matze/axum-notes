# Authorization using tower-http

In this example we will use tower-http's [auth module][] to add a [custom][]
authorization layer. There are also two builtin `bearer` and `password` layers
however since both bearer token and username and password combo are set at
compile time, the use is somewhat limited.

In this case we implement the `AuthorizeRequest` trait for our custom `Auth`
struct, which requires to set the appropriate `ResponseBody` (which in axum's
case _must be_ `axum::body::BoxBody`) as well as an `authorize` method. Here,
based on the request and any data on the struct itself we can make authorization
decisions. In this example we extract the `Authorization` header field and
compare it with a pre-defined value. If it matches we can return `Ok(())` and
the route is taken otherwise we return an error response.


## Dependencies

```toml
{{#include Cargo.toml:6:}}
```


## Code

```rust
{{#include src/main.rs}}
```


## Run

Start the server with

```
cargo run --bin auth-with-tower-http
```

Trying to get the root route without setting an authorization header results in
a 401:

```
$ curl -i http://127.0.0.1:8080
HTTP/1.1 401 Unauthorized
content-length: 0
date: Tue, 08 Mar 2022 19:57:15 GMT
```

On the other hand, setting `Authorization` appropriately succeeds:

```
$ curl -i -H "Authorization: lol" http://127.0.0.1:8080
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 5
date: Tue, 08 Mar 2022 19:59:15 GMT

hello
```

[auth module]: https://docs.rs/tower-http/latest/tower_http/auth/index.html
[custom]: https://docs.rs/tower-http/latest/tower_http/auth/struct.RequireAuthorizationLayer.html#method.custom
