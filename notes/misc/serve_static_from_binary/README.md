# Serve static data from the binary

For very small (micro) services it can come in handy to just distribute a single
binary containing both code and data such as CSS or Javascript and avoid the
hassle of dealing with paths, permissions, deployment etc. In this example we
use the handy [include_dir][] crate to bundle a directory of data within the
compiled binary and the [mime_guess][] crate to guess a MIME type based on the
served file.


## Dependencies

```toml
{{#include Cargo.toml:6:}}
```


## Code

First of all create a `static` directory next to the `src` directory and add
this sample CSS file `foo.css`:

```css
{{#include static/foo.css}}
```

The interesting part of the code are the route which matches _any_ path prefixed
`/static` and the handler. In the handler we first strip the initial slash
(because that would not match with `get_file`) and then just try to load the
file. If we have a match, we try to guess a suitable MIME type and return it,
otherwise we just return a 404:

```rust,noplayground
{{#include src/main.rs}}
```


## Run

Start the server with

```
cargo run --bin serve-static-from-binary
```

and it will serve `/static/foo.css` as expected

```
$ curl http://127.0.0.1:8080/static/foo.css
body {
    background-color: #ccc;
}
```

[include_dir]: https://crates.io/crates/include_dir
[mime_guess]: https://crates.io/crates/mime_guess
