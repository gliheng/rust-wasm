OUTPUT=static
TARGET=wasm32-unknown-emscripten

all:
	cargo build --target=$(TARGET)
	mkdir -p $(OUTPUT)
	find target/wasm32-unknown-emscripten/debug/deps -type f -name "*.wasm" | xargs -I {} cp {} $(OUTPUT)/app.wasm
	find target/wasm32-unknown-emscripten/debug/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} cp {} $(OUTPUT)/app.js

server:
	cd $(OUTPUT) && python3 -m http.server
