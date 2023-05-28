release:
	cargo build --release
docs:
	cargo doc --open
expand:
	cargo +nightly expand
fmt:
	cargo fmt
prepare:
	cargo sqlx prepare -- --lib
test:
	make fmt
	cargo test
run:
	make test
	cargo run