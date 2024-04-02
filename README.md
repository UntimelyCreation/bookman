# bookman

A simple, local Web service for managing and sharing bookmarks between your Web browsers.

`bookman` aims to be minimal and fast: bookmarks are stored in a local SurrealDB database, and are managed from a local Web page built using Rust with `axum` and `htmx`.

**IMPORTANT**: This project is at an early development stage. Although technically functional at a very basic level, it is not yet intended to be used.


## Usage

### Prerequisites

A working installation of [Rust](https://rustup.rs/) and [SurrealDB](https://surrealdb.com/) is required.


### Build and run from source

Currently, the `surrealdb` service must first be launched manually with the following command:

```bash
surreal start memory --log trace --user root --pass root
```

Then, launch `bookman` with the usual `cargo` command:

```bash
cargo run --release
```

The `bookman` Web page should then be accessible from a Web browser, through the following URL:

```
http://localhost:3000/
```


## License

Distributed under the MIT License.
