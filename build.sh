cargo build --release
mv ./target/release/swordfish bin/linux/
cargo build --release --target x86_64-pc-windows-gnu
mv ./target/x86_64-pc-windows-gnu/release/swordfish.exe bin/windows/

