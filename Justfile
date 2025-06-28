nixdev := "nix develop -c"
fast:
        {{nixdev}} cargo run --release
slow:
        {{nixdev}} cargo run

build:
        {{nixdev}} cargo build --release
bench:
        {{nixdev}} cargo bench
test:
        {{nixdev}} cargo test
