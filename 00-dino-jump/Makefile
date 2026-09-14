GAME := dino-jump

.PHONY: wasm redist serve all clean

wasm:
	cargo build --release --target wasm32-unknown-unknown

redist: wasm
	cp target/wasm32-unknown-unknown/release/$(GAME).wasm redist/

serve: redist
	python3 -m http.server 8080 --directory redist

all: wasm redist serve

clean:
	cargo clean
	rm -f redist/*.wasm
