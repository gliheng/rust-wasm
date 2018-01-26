OUTPUT=static

all:
	cargo build --target=wasm32-unknown-emscripten --release
	mkdir -p $(OUTPUT)
	find target/wasm32-unknown-emscripten/release/deps -type f -name "*.wasm" | xargs -I {} cp {} $(OUTPUT)/
	find target/wasm32-unknown-emscripten/release/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} cp {} $(OUTPUT)/app.js

server:
	cd $(OUTPUT) && python3 -m http.server
