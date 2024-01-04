.PHONY: run
run:
	python3 -m http.server

.PHONY: build
build:
	wasm-pack build --target web --out-name wasm_matrix_calculator