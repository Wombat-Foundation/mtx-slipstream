SHELL=/bin/bash
.DEFAULT_GOAL := _help

MAKEFLAGS += --no-print-directory

CARGO ?= cargo

.PHONY: _help
_help:
	@grep -E '^[a-zA-Z_/%-]+:.*?##H' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?##H "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'



.PHONY: format
format: check-cargo-sort ##H Format code
	prettier -w $$(git ls-files '*.md' '*.y*ml')
	pre-commit run --all-files
	$(CARGO) +nightly fmt --all
	$(CARGO) sort --workspace --grouped

.PHONY: check-cargo-sort
check-cargo-sort:
	@command -v cargo-sort >/dev/null || { echo "cargo-sort is required; install it with: cargo install cargo-sort" >&2; exit 1; }



.PHONY: all
all:	##H Run all main targets
all: format
	$(MAKE) check doc lint test


.PHONY: check
check: ##H Type-check without building
	$(CARGO) check --all-targets --all-features

.PHONY: macro
macro: ##H See macro expansion costs
	$(CARGO) +nightly rustc -- -Zmacro-stats 2>&1 | awk -f scripts/macro_totals.awk

.PHONY: lint
lint: ##H Run clippy lints
	$(CARGO) clippy --all-targets --all-features -- $(if $(CI),-D warnings)

.PHONY: fix
fix: ##H Apply auto-fixes with clippy
	$(CARGO) clippy --fix --allow-dirty --allow-staged --allow-no-vcs --all-targets



.PHONY: doc
doc: ##H Build docs
	$(CARGO) test --doc
	$(CARGO) doc --no-deps
	echo '<meta http-equiv="refresh" content="0;url=mtx_slipstream/index.html">' > target/doc/index.html



.PHONY: test
test: ##H Run tests
	$(CARGO) test --lib --tests --timings --all-targets --all-features

.PHONY: cov
cov: ##H Run code coverage and generate HTML report
	# TODO: include `src/bin/` in coverage
	# Run coverage
	$(CARGO) llvm-cov --lib --tests \
		--all-targets --all-features \
		--html --output-dir .coverage \
		--ignore-filename-regex 'src/bin/.*|scripts/.*'
	# Process report to codecov-compatible JSON
	$(CARGO) llvm-cov report \
		--ignore-filename-regex 'src/bin/.*|scripts/.*' \
		--codecov --output-path .coverage/codecov.json
	@echo DONE. You may open it with:
	@echo firefox .coverage/html/index.html



.PHONY: build
build: ##H Build the lib/binary
	cargo build --release --timings --all-targets --all-features



.PHONY: bench
bench: ##H Run benchmarks (p=<bench-name>, default: all)
	$(CARGO) +nightly bench --manifest-path benches/Cargo.toml --features bench $(if $(p),--bench $(p) $(if $(filter serde_cmp,$(p)),--features serde-comparison))

.PHONY: clean
clean: ##H Clean build artifacts
	$(CARGO) clean
