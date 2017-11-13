OUTPUT=static
TARGET=wasm32-unknown-emscripten

All:
	cargo build --target=wasm32-unknown-emscripten

server:
	cd $(OUTPUT) && python3 -m http.server

