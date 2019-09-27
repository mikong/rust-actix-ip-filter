# IP Address Filter

IP Address Filter using Actix Web is a Rust programming exercise.

## Database Setup

Install Diesel CLI tool:

```
$ cargo install diesel_cli
```

Create database:

```
$ diesel database setup
```

## Usage

### Running the HTTP Server

```bash
$ cargo run --bin actix_ip_filter
```

### Running the Admin CLI

```bash
$ cargo run --bin cli
```

Commands:

* `ADD <IP>`
* `REMOVE <IP>`
* `EXIT` or `QUIT`

For example, to add an IP address to filter:

```
> ADD 192.168.0.1
> EXIT
```

## Websocket Client

Open http://localhost:8088/client/index.html. The HTML from the actix websocket
example was used.

## License

This software is distributed under the [MIT License][license].

[license]: https://github.com/mikong/rust-actix-ip-filter/blob/master/LICENSE
