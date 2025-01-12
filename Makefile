all: wasm cli

run:
	cargo run

test:
	cargo test

cli:
	cargo build --release

wasm:
	$(MAKE) -C rusty-life-wasm

clean:
	rm -rf target
	$(MAKE) -C rusty-life-wasm clean

.PHONY: wasm cli run clean test