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
	cargo test
run:
	make test
	cargo run