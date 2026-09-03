SHELL=/bin/bash
.DEFAULT_GOAL := _help

MAKEFLAGS += --no-print-directory

CARGO ?= cargo

.PHONY: _help
_help:
	@grep -E '^[a-zA-Z_/%-]+:.*?##H' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?##H "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'



.PHONY: format
format: ##H Format code
	-prettier -w $$(git ls-files '*.md' '*.y*ml')
	pre-commit run --all-files
	$(CARGO) sort --workspace --grouped
	$(CARGO) fmt --all
	$(CARGO) clippy --fix --allow-dirty --allow-staged --allow-no-vcs --all-targets --all-features



.PHONY: check
check: ##H Type-check without building
	$(CARGO) check --all-targets --all-features

.PHONY: lint
lint: ##H Run clippy lints
	$(CARGO) clippy --all-targets --all-features -- -D warnings



.PHONY: doc
doc: ##H Build docs
	$(CARGO) test --doc
	$(CARGO) doc --no-deps
	echo '<meta http-equiv="refresh" content="0;url=rezzy/index.html">' > target/doc/index.html



.PHONY: test
test: ##H Run tests
	$(CARGO) test --all-targets --all-features



.PHONY: clean
clean: ##H Clean build artifacts
	$(CARGO) clean
