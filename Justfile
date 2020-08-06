default: build

build: check
  @cargo build

check:
  @cargo check
  @cargo +nightly clippy

build_release: check
  @cargo build --release

test:
  @cargo test
