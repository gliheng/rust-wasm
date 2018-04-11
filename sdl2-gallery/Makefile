OUTPUT=static
TARGET=wasm32-unknown-emscripten

all:
	cargo build --target=$(TARGET)
	mkdir -p $(OUTPUT)
	find target/wasm32-unknown-emscripten/debug/deps -type f -name "*.wasm" | xargs -I {} mv {} $(OUTPUT)/
	find target/wasm32-unknown-emscripten/debug/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} mv {} $(OUTPUT)/app.js
	find target/wasm32-unknown-emscripten/debug/deps -type f -name "*.data" | xargs -I {} mv {} $(OUTPUT)/

server:
	cd $(OUTPUT) && python3 -m http.server

clean:
	rm static/app.js static/sdl2_gallery.wasm

release:
	cargo build --target=$(TARGET) --release
	mkdir -p $(OUTPUT)
	find target/wasm32-unknown-emscripten/release/deps -type f -name "*.wasm" | xargs -I {} mv {} $(OUTPUT)/
	find target/wasm32-unknown-emscripten/release/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} mv {} $(OUTPUT)/app.js
	find target/wasm32-unknown-emscripten/release/deps -type f -name "*.data" | xargs -I {} mv {} $(OUTPUT)/
	wasm-gc ./static/sdl2_gallery.wasm
