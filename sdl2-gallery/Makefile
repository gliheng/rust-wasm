OUTPUT=static
TARGET=wasm32-unknown-emscripten

all:
	cargo build --target=$(TARGET)
	mkdir -p $(OUTPUT)
	find target/wasm32-unknown-emscripten/debug/deps -type f -name "*.wasm" | xargs -I {} mv {} $(OUTPUT)/app.wasm
	find target/wasm32-unknown-emscripten/debug/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} mv {} $(OUTPUT)/app.js
	find target/wasm32-unknown-emscripten/debug/deps -type f -name "*.data" | xargs -I {} mv {} $(OUTPUT)/

server:
	cd $(OUTPUT) && python3 -m http.server
