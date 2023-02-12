test:
	cargo clippy
	cargo build
	cat test.json |RUST_BACKTRACE=1 ./target/debug/jsonify --no-color
	cat test.json |RUST_BACKTRACE=1 ./target/debug/jsonify 
	echo "Blah" |RUST_BACKTRACE=1 ./target/debug/jsonify 

release:
	cargo build --target=aarch64-apple-darwin --release
	cargo build --target=x86_64-apple-darwin --release

install: release
	mkdir -p ~/bin
	cp ./target/release/jsonify ~/bin/
