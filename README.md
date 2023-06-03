# rust-newsletter

[![Rust](https://github.com/hazadus/rust-newsletter/actions/workflows/general.yml/badge.svg?branch=master)](https://github.com/hazadus/rust-newsletter/actions/workflows/general.yml)

Simple newsletter app, written in Rust.

Based on Luca Palmieri's [awesome book](http://library.hazadus.ru/books/45/details/) with some improvements.

## References
 - [Actix Documentation](https://actix.rs/docs)
 - [sqlx repo](https://github.com/launchbadge/sqlx)
 - [Serde docs](https://serde.rs)
 - [reqwest](https://crates.io/crates/reqwest)
 - [env_logger docs](https://docs.rs/env_logger/latest/env_logger/)
 - [log docs](https://docs.rs/log/latest/log/)
 - [tracing docs](https://docs.rs/tracing/latest/tracing/) - `tracing` is a framework for instrumenting Rust programs with context-aware, structured, event-based diagnostic information.
 - [tracing_bunyan_formatter docs](https://docs.rs/tracing-bunyan-formatter/0.1.6/tracing_bunyan_formatter/)
 - [tracing_log docs](https://docs.rs/tracing-log/latest/tracing_log/)
 - [secrecy docs](https://docs.rs/secrecy/latest/secrecy/)
 - [tracing-actix-web](https://github.com/LukeMathWalker/tracing-actix-web/tree/main)
 - [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)
 - [validator](https://crates.io/crates/validator)
 - [fake](https://crates.io/crates/fake)
 - [quickcheck](https://crates.io/crates/quickcheck)
 - [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs)
 - [Postmark API Reference - Sending a single email](https://postmarkapp.com/developer/user-guide/send-email-with-api#send-a-single-email)

### Starting app in dev mode

Run the PostgreSQL container and apply the migration:

```bash
./scripts/init_db.sh
```

Start the app:

```bash
cargo run
```

Or, see `Makefile` for `make` commands.

### Tests

```bash
./scripts/init_db.sh
TEST_LOG=true cargo test | bunyan
```

### Tooling

#### Docker

```bash
docker build --tag newsletter --file Dockerfile .
docker run -p 8000:8000 newsletter
# Check it is working:
curl -v http://127.0.0.1:8000/health_check
# Push image to the Docker Hub:
docker tag newsletter hazadus/rust-newsletter
docker push hazadus/rust-newsletter
```

#### Working with `sqlx`

Install `sqlx-cli`:

```bash
# Note the version
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
# Check installation
sqlx --help
# Prepare offline DB data
cargo sqlx prepare -- --lib
# Check offline data
cargo sqlx prepare --check -- --bin newsletter
```

#### Using `cargo-udeps`

```bash
cargo install cargo-udeps
cargo +nightly udeps
```

#### Test logs pretty print

```bash
cargo install bunyan
TEST_LOG=true cargo test | bunyan
```