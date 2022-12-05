# vi ~/.cargo/config  
# [target.aarch64-unknown-linux-gnu]
# linker = "aarch64-linux-gnu-gcc"
rustup target add aarch64-unknown-linux-gnu
cargo build --release  --target=aarch64-unknown-linux-gnu
