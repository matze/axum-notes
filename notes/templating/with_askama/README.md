# Templating using askama

[Askama][] implements a type-safe, Jinja-like template engine. While the templating engine itself
is independent of any particular application, the `askama_axum` crate provides additional
integration points by converting template results to response objects. Because Askama evaluates
templates at compile time, it is both safe to use and fast to evaluate. On the other hand, it
cannot be fed with templates by users at run-time.


## Dependencies

```toml
{{#include Cargo.toml:6:}}
```


## Code

First create a new sibling directory `templates` next to the `src` directory and add an
`index.html`:

```html
{{#include templates/index.html}}
```

Note that we refer to the two template variables `name` and `age`. Thanks to the compile-time
guarantees of askama, the compiler will complain if we do not derive a template that contains these
variables:

```rust
{{#include src/main.rs}}
```


## Run

Start the server with

```
cargo run --bin templating-with-askama
```

Now, lets try to get `/john/32`:

```
$ curl -i http://127.0.0.1:8080/john/32
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 167
date: Tue, 08 Mar 2022 20:13:10 GMT

<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Askama</title>
  </head>

  <body>
    <p>Hello john, you are 32 years old.</p>
  </body>
</html>
```

[Askama]: https://docs.rs/askama/latest/askama/
