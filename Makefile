install:
	cargo build --release
	sudo rm /usr/local/bin/autorunner
	sudo cp target/release/autorunner /usr/local/bin/