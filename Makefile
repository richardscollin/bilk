all: check lint typos coverage

install-deps:
	rustup update
	rustup component add llvm-tools-preview rustfmt
	cargo install cargo-llvm-cov cargo-nextest cargo-sort typos-cli
run:
	cargo run
test:
	cargo nextest
check:
	cargo check --all-targets
bench:
	cargo bench
coverage:
	cargo llvm-cov --html nextest --open
clean:
	cargo clean
doc:
	cargo doc --no-deps --open
lint:
	cargo clippy
typos:
	typos
sort-toml:
	cargo sort --grouped 
fmt:
	cargo +nightly fmt
