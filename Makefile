prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo +nightly build --release -p did --target wasm32-unknown-unknown
	cargo +nightly build --release -p demovcregistry --target wasm32-unknown-unknown
	
test-only:
	cargo test -p did_tests -- --nocapture --test-threads=1
	cargo test -p demovcregistry_tests -- --nocapture --test-threads=1

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/did.wasm tests/did/wasm
	cp target/wasm32-unknown-unknown/release/demovcregistry.wasm tests/demovcregistry/wasm

test: build-contract copy-wasm-file-to-test test-only


clean:
	cargo clean
	rm -rf tests/wasm/contract.wasm
