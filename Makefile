release:
	cargo build --release
docs:
	cargo doc --open
expand:
	cargo +nightly expand
fmt:
	cargo fmt
test:
	make fmt
	./scripts/init_db.sh
	cargo test
run:
	make test
	cargo run