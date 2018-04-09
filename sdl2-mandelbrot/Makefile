OUTPUT=static/public/
TARGET=wasm32-unknown-emscripten

all:
	cargo build --release --target=$(TARGET)
	mkdir -p $(OUTPUT)
	find target/wasm32-unknown-emscripten/release/deps -type f -name "*.wasm" | xargs -I {} mv {} $(OUTPUT)/
	find target/wasm32-unknown-emscripten/release/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} mv {} $(OUTPUT)/
	find target/wasm32-unknown-emscripten/release/deps -type f -name "*.data" | xargs -I {} mv {} $(OUTPUT)/

debug:
	cargo build --target=$(TARGET)
	mkdir -p $(OUTPUT)
	find target/wasm32-unknown-emscripten/debug/deps -type f -name "*.wasm" | xargs -I {} mv {} $(OUTPUT)/
	find target/wasm32-unknown-emscripten/debug/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} mv {} $(OUTPUT)/
	find target/wasm32-unknown-emscripten/debug/deps -type f -name "*.data" | xargs -I {} mv {} $(OUTPUT)/

clean:
	rm static/app.js static/sdl2_gallery.wasm

release:
	wasm-gc ./static/sdl2_gallery.wasm

serve:
	cd static && npm run serve
