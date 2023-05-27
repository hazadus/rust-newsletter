# rust-newsletter

Simple newsletter app, written in Rust.

Base on Luca Palmieri's [awesome book](http://library.hazadus.ru/books/45/details/).

## References
 - [Actix Documentation](https://actix.rs/docs)
 - [sqlx repo](https://github.com/launchbadge/sqlx)
 - [Serde docs](https://serde.rs)
 - [env_logger docs](https://docs.rs/env_logger/latest/env_logger/)
 - [log docs](https://docs.rs/log/latest/log/)
 - [tracing docs](https://docs.rs/tracing/latest/tracing/) - `tracing` is a framework for instrumenting Rust programs with context-aware, structured, event-based diagnostic information.
 - [tracing_bunyan_formatter docs](https://docs.rs/tracing-bunyan-formatter/0.1.6/tracing_bunyan_formatter/)
 - [tracing_log docs](https://docs.rs/tracing-log/latest/tracing_log/)

### Working with `sqlx`

Install `sqlx-cli`:

```bash
# Note the version
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
# Check installation
sqlx --help
```

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