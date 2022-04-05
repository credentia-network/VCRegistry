prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo +nightly build --release -p did --target wasm32-unknown-unknown
	cargo +nightly build --release -p demovcregistry --target wasm32-unknown-unknown
	
clean:
	cargo clean
	rm -rf tests/wasm/contract.wasm
