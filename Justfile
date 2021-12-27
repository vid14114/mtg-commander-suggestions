# just manual: https://github.com/casey/just/#readme
# discussion on cross-platform justfiles: https://github.com/casey/just/issues/1050

# requires installed cargo subcommands: cargo-outdated cargo-udeps

_default:
    @just --list

# Runs clippy on the sources 
check:
	cargo clippy --locked -- -D warnings

# Runs unit tests
test:
	cargo test --locked

# Finds unused dependencies (requires installed rust nightly toolchain)
udeps:
    cargo +nightly udeps --all-targets --backend depinfo

# Finds out of date dependencies
outdated:
    cargo outdated