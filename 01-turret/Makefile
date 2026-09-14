GAME := turret

.PHONY: wasm redist serve all clean

wasm:
	cargo build --release --target wasm32-unknown-unknown

redist: wasm
	cp target/wasm32-unknown-unknown/release/$(GAME).wasm redist/
	mkdir -p redist/assets && cp -R assets/. redist/assets/

serve: redist
	python3 -m http.server 8080 --directory redist

all: wasm redist serve

clean:
	cargo clean
	rm -f redist/*.wasm
	rm -rf redist/assets
