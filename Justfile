# just manual: https://github.com/casey/just/#readme
# discussion on cross-platform justfiles: https://github.com/casey/just/issues/1050

_default:
    @just --list

# Runs clippy on the sources 
check:
	cargo clippy --locked -- --deny warnings

# Runs unit tests
test:
	cargo test --locked

# Finds unused dependencies (requires installed rust nightly toolchain and cargo-outdated)
udeps:
    cargo +nightly udeps --all-targets --backend depinfo

# Finds out of date dependencies (requires cargo-outdated)
outdated:
    cargo outdated --root-deps-only