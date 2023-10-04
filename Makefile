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

.PHONY: build
build: ## Build application (in release mode)
	@$(MAKE) -s $(TARGET_APP)

.PHONY: build-watch
build-watch: ## Automatic execution upon updates (in release mode)
	@find . -type f -name '*.rs' | entr -c -s "$(MAKE) -s build"

.PHONY: run
run: ## Run a release version
	@$(TARGET_APP)
