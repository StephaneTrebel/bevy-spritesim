APP_NAME := $(shell grep "name = " Cargo.toml | cut -d'"' -f2)
SOURCES := $(shell find . -type f -name "*.rs")
TARGET_APP := target/release/$(APP_NAME)

.DEFAULT: help

.PHONY: help
help:
	@grep -E '^[///a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: install
install: ## Install dependencies
	@echo no deps yet

.PHONY: check
check: ## Check code
	@cargo check

$(TARGET_APP): $(SOURCES) ## Release version of the app
	@cargo build --release

.PHONY: build-release
build-release: ## Build application
	@$(MAKE) -s $(TARGET_APP)

.PHONY: build-dev-watch
build-dev-watch: ## Automatic execution upon updates
	@find src -type f | entr -r -s "cargo run -q"

.PHONY: run
run: ## Run the built app
	@$(TARGET_APP)
