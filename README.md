# rust-http-server

An HTTP server written in Rust on top of TCP sockets.

It accepts a connection, parses the request line, and responds with `200 OK` for `/` and `404 Not Found` for every other path.

## Requirements

- Rust 1.96 (`cargo`)

## Run

```sh
cargo run
```

The server listens on `127.0.0.1:4221`.

## Try it

```sh
curl -i http://127.0.0.1:4221/
curl -i http://127.0.0.1:4221/missing
```
