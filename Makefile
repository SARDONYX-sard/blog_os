default: run

fmt:
		cargo fmt --all -- --check

run:
		cargo run

build:
		cargo build --target x86_64-blog_os.json

test:
		cargo test

# need `cargo install bootimage` & `rustup component add llvm-tools-preview`
mk-boot-img:
		cargo bootimage

boot: mk-boot-img
		qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
