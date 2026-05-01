# rust-redis

My own implementation of Redis, written in Rust.

## Features

- RESP protocol parser
- TCP server with async connection handling
- GET / SET / DEL commands
- TTL / key expiration
- AOF persistence

## Getting Started

### Prerequisites

Ensure you have the following installed on your system:
- [Rust](https://rustup.rs/) 1.75 or later (stable toolchain)
- `redis-cli` (optional, for manual testing)

### Building

```bash
git clone https://github.com/ggjorven/rust-redis.git
cd rust-redis/redis
cargo build --release
```

### Running

```bash
cargo run --release
```

The server listens on `127.0.0.1:6379` by default. You can connect with any Redis client:

```bash
redis-cli SET foo bar
redis-cli GET foo
```

## License

This project is licensed under the GNU General Public License. See [LICENSE](LICENSE.txt) for details.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## Third-Party Libraries
- [XXXX](xxxxxx) - xxxxxx