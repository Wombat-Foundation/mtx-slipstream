SHELL=/bin/bash
.DEFAULT_GOAL := help

MAKEFLAGS += --no-print-directory

.PHONY: help
help: ##H Show this help
	@grep -E '^[a-zA-Z_/%-]+:.*?##H' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?##H "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: fmt
fmt: ##H Format code
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ##H Check formatting (CI)
	cargo fmt --all -- --check

.PHONY: lint
lint: ##H Run clippy lints
	cargo clippy --all-targets -- -D warnings

.PHONY: lint-fix
lint-fix: ##H Run clippy and auto-fix
	cargo clippy --fix --allow-dirty --allow-no-vcs

.PHONY: test
test: ##H Run tests
	cargo test --all-targets

.PHONY: test-doc
test-doc: ##H Run doc tests
	cargo test --doc

.PHONY: check
check: ##H Type-check without building
	cargo check --all-targets

.PHONY: pre-commit
pre-commit: ##H Run pre-commit hooks
	pre-commit run --all-files

.PHONY: ci
ci: fmt-check lint test ##H Full CI pipeline (fmt + lint + test)

.PHONY: clean
clean: ##H Clean build artifacts
	cargo clean
