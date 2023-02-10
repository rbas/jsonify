test:
	cargo build
	cat test.json |RUST_BACKTRACE=1 ./target/debug/jsonify 
	echo "Blah" |RUST_BACKTRACE=1 ./target/debug/jsonify 