Run Validator:
solana-test-validator

Set config:
solana config set --url localhost


Build program:
cargo build-bpf

Deploy program:
solana program deploy ./target/deploy/hello_world.so


Continue on: ..https://solana.com/developers/guides/getstarted/local-rust-hello-world