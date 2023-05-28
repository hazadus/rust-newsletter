release:
	cargo build --release
docs:
	cargo doc --open
expand:
	cargo +nightly expand
fmt:
	cargo fmt
	cargo clippy -- -D warnings
prepare:
	cargo sqlx prepare -- --lib
test:
	make fmt
	cargo test
run:
	make test
	cargo run
up:
	make prepare
	make test
	docker compose up --build