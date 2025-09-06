install:
	cargo clean
	cargo build --release
	sudo cp target/release/autorunner /usr/local/bin/