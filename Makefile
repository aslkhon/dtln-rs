# Makefile
.PHONY: build-nodejs build-python clean

# Build for Node.js
build-nodejs:
	npm run install-native

# Build for Python
build-python:
	maturin develop --features=python

# Build Python wheel
build-python-wheel:
	maturin build --release --features=python --interpreter python3.11 python3.12 python3.13

# Clean all builds
clean:
	cargo clean
	rm -f index.node
	rm -f dtln.js

# Install Python dependencies
install-python-deps:
	pip install maturin

# Development setup
dev-setup:
	npm install
	pip install maturin