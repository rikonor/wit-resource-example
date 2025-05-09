all: extension facade host

clean:
	@rm -r target || :

.PHONY: extension
extension:
	@cargo build --target wasm32-wasip2 -p extension

.PHONY: facade
facade:
	@cargo build --target wasm32-wasip2 -p facade

.PHONY: host
host:
	@cargo run -p host
