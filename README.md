# HTTP Server in Rust with Axum

You can launch the HTTP server at http://localhost:8000 with the following command:

```bash
cargo run

# in another terminal, you can test the API with
curl http://localhost:8000 -v
curl http://localhost:8000/health -v
```

To have a better experience while developing, you can use [cargo-watch](https://crates.io/crates/cargo-watch). It will automatically recompile and restart the server when you change a file.
```bash
cargo install cargo-watch
cargo watch -x run

# to test and run the server
cargo watch -x test -x run
```

To have bunyan logs, you can install [bunyan](https://github.com/LukeMathWalker/bunyan)
```bash
cargo install bunyan

# to run the server with bunyan logs
cargo watch -x run | bunyan
```
