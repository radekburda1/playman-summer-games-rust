PROJECT_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
REFERENCE_DIR ?= $(PROJECT_DIR)/../reference-runner
CARGO ?= cargo
JAR ?= /Users/Radek.Burda/Downloads/playmansum_2jvrll9n.jar
PORT ?= 8765
WEB_PORT ?= 8767
WEB_CARGO ?= $(CARGO)
WEB_ENV ?=

# Optional developer-only toolchain/cache overrides; never required by a checkout.
-include $(PROJECT_DIR)/.make.local

RUST_ARGS ?=

.DEFAULT_GOAL := help
.PHONY: help rust rust-direct mac-app java original build test web web-build

help:
	@echo 'make rust      Run the Rust game (macOS app launch; release build)'
	@echo 'make mac-app   Build dist/Playman Rust.app on macOS'
	@echo 'make rust-direct Run the executable directly for startup diagnostics'
	@echo 'make java      Run the original Java game; open http://127.0.0.1:$(PORT)'
	@echo 'make web       Build and serve the Rust browser version on port $(WEB_PORT)'
	@echo 'make web-setup Install the WASM target for the active Rust toolchain'
	@echo 'make compare   Export reference-input replay frames without a graphics context'
	@echo 'make build     Build the Rust game'
	@echo 'make test      Run Rust tests'
	@echo 'Overrides: JAR=/path/game.jar PORT=8766 PLAYMAN_JAVA=/path/to/java'
	@echo '           REFERENCE_DIR=/path/to/reference-runner CARGO=/path/to/cargo'

rust:
ifeq ($(shell uname -s),Darwin)
	$(MAKE) mac-app
	open -n "$(PROJECT_DIR)/dist/Playman Rust.app" --args $(RUST_ARGS)
else
	$(MAKE) rust-direct
endif

rust-direct:
	cd "$(PROJECT_DIR)" && $(CARGO) run --release --locked -- $(RUST_ARGS)

mac-app:
	@test "$$(uname -s)" = Darwin || { echo 'mac-app requires macOS; use make rust instead'; exit 1; }
	cd "$(PROJECT_DIR)" && $(CARGO) build --release --locked --bin playman-rust
	cd "$(PROJECT_DIR)" && $(CARGO) metadata --locked --no-deps --format-version 1 | python3 tools/package_macos.py

java:
	@test -f "$(JAR)" || { echo 'Game JAR not found. Set JAR=/absolute/path/to/game.jar'; exit 1; }
	@test -f "$(REFERENCE_DIR)/run.sh" || { echo 'Reference runner not found. Set REFERENCE_DIR=/absolute/path/to/reference-runner'; exit 1; }
	@echo 'Original game viewer: http://127.0.0.1:$(PORT) (Ctrl+C to stop)'
	PLAYMAN_JAVA="$(PLAYMAN_JAVA)" "$(REFERENCE_DIR)/run.sh" "$(JAR)" "$(PORT)"

original: java

build:
	cd "$(PROJECT_DIR)" && $(CARGO) build --release --locked

test:
	cd "$(PROJECT_DIR)" && $(CARGO) test --locked

# One-time prerequisite: rustup target add wasm32-unknown-unknown
web-build:
	cd "$(PROJECT_DIR)" && $(WEB_ENV) $(WEB_CARGO) build --bin playman-rust --release --locked --target wasm32-unknown-unknown
	cd "$(PROJECT_DIR)" && $(WEB_ENV) $(WEB_CARGO) metadata --locked --format-version 1 | python3 tools/prepare_web.py

web: web-build
	@echo 'Rust browser game: http://127.0.0.1:$(WEB_PORT) (Ctrl+C to stop)'
	python3 -m http.server "$(WEB_PORT)" --bind 127.0.0.1 --directory "$(PROJECT_DIR)/web"

.PHONY: web-setup compare
web-setup:
	rustup target add wasm32-unknown-unknown

compare:
	cd "$(PROJECT_DIR)" && $(CARGO) run --locked --bin compare -- analysis/hurdles/rust
