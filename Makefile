test:
	cargo test -- --nocapture
	cargo test -- --ignored -- --nocapture

miri:
	cargo miri test -- --nocapture

bench:
	cargo bench -- --nocapture

lint:
	cargo fmt
	cargo clippy -- -D warnings

build:
	cargo build --release

clean:
	cargo clean

.PHONY: test bench lint build clean miri
