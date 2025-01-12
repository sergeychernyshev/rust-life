wasm:
	$(MAKE) -C rusty-life-wasm

clean:
	rm -rf target
	$(MAKE) -C rusty-life-wasm clean

.PHONY: wasm