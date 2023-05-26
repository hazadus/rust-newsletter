# rust-newsletter

Simple newsletter app, written in Rust.

## References
 - [Actix Documentation](https://actix.rs/docs)
 - [sqlx repo](https://github.com/launchbadge/sqlx)
 - [Serde docs](https://serde.rs)
 - [env_logger docs](https://docs.rs/env_logger/latest/env_logger/)
 - [log docs](https://docs.rs/log/latest/log/)
 - [tracing docs](https://docs.rs/tracing/latest/tracing/)

### Working with `sqlx`

Install `sqlx-cli`:

```bash
# Note the version
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
# Check installation
sqlx --help
```