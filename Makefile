.PHONY: dev release clean serve

# Set environment for WASM builds
export RUSTFLAGS = -C target-feature=+atomics,+bulk-memory,+mutable-globals

# Fast development build
dev:
	wasm-pack build worker-crate --target no-modules --dev --no-opt --out-dir ../webroot/pkg
	wasm-pack build main-crate --target web --dev --no-opt --out-dir ../webroot/pkg

# Optimized release build (slow but small)
release:
	wasm-pack build worker-crate --target no-modules --release --out-dir ../webroot/pkg
	wasm-pack build main-crate --target web --release --out-dir ../webroot/pkg

# Clean build artifacts
clean:
	rm -rf webroot/pkg
	cd worker-crate && cargo clean
	cd main-crate && cargo clean
	cd protocol-crate && cargo clean

# Start dev server
serve:
	simple-http-server --coop --coep --nocache webroot

# Quick dev build and serve
run: dev serve
