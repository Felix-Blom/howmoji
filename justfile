clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

test:
    cargo test