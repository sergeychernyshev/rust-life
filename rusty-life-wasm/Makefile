web: ./pkg/rusty_life_wasm.js

pkg/rusty_life_wasm.js: src/lib.rs ../rusty-life-lib/src/lib.rs
	wasm-pack build --target web

clean:
	rm -rf pkg

.PHONY: web clean