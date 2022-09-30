export CG_RUSTFLAGS="-C linker=ld.lld-14 -C link-args=--no-pie -C link-args=--no-dynamic-linker"
~/rustc_codegen_gcc/cargo.sh b --target i686-unknown-linux-gnu
strip ./target/i686-unknown-linux-gnu/debug/hello_world
objcopy -j.text -j.rodata ./target/i686-unknown-linux-gnu/debug/hello_world
