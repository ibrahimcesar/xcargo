# xcargo Makefile
# Cross-compilation, zero friction 🎯

# Colors
BOLD := \033[1m
RESET := \033[0m
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
MAGENTA := \033[35m
CYAN := \033[36m

.PHONY: help
help: ## 📚 Show this help message
	@echo "$(BOLD)$(CYAN)xcargo - Cross-compilation, zero friction 🎯$(RESET)"
	@echo ""
	@echo "$(BOLD)Available commands:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""

# ========================================
# 🦀 Rust/Cargo Commands
# ========================================

.PHONY: build
build: ## 🔨 Build the project
	@echo "$(BOLD)$(BLUE)🔨 Building xcargo...$(RESET)"
	cargo build

.PHONY: build-release
build-release: ## 🚀 Build in release mode
	@echo "$(BOLD)$(GREEN)🚀 Building xcargo (release mode)...$(RESET)"
	cargo build --release
	@echo "$(GREEN)✅ Release build complete!$(RESET)"

.PHONY: test
test: ## 🧪 Run tests
	@echo "$(BOLD)$(YELLOW)🧪 Running tests...$(RESET)"
	cargo test

.PHONY: test-verbose
test-verbose: ## 🔍 Run tests with verbose output
	@echo "$(BOLD)$(YELLOW)🔍 Running tests (verbose)...$(RESET)"
	cargo test -- --nocapture --test-threads=1

.PHONY: check
check: ## ✅ Check code without building
	@echo "$(BOLD)$(CYAN)✅ Checking code...$(RESET)"
	cargo check

.PHONY: clippy
clippy: ## 📎 Run clippy lints
	@echo "$(BOLD)$(MAGENTA)📎 Running clippy...$(RESET)"
	cargo clippy -- -D warnings

.PHONY: fmt
fmt: ## 🎨 Format code
	@echo "$(BOLD)$(CYAN)🎨 Formatting code...$(RESET)"
	cargo fmt

.PHONY: fmt-check
fmt-check: ## 🔍 Check code formatting
	@echo "$(BOLD)$(CYAN)🔍 Checking formatting...$(RESET)"
	cargo fmt -- --check

.PHONY: clean
clean: ## 🧹 Clean build artifacts
	@echo "$(BOLD)$(RED)🧹 Cleaning build artifacts...$(RESET)"
	cargo clean
	@echo "$(GREEN)✅ Clean complete!$(RESET)"

.PHONY: run
run: ## 🏃 Run xcargo
	@echo "$(BOLD)$(GREEN)🏃 Running xcargo...$(RESET)"
	cargo run

.PHONY: run-example
run-example: ## 📋 Run target_info example
	@echo "$(BOLD)$(CYAN)📋 Running target_info example...$(RESET)"
	cargo run --example target_info

.PHONY: install
install: ## 📦 Install xcargo locally
	@echo "$(BOLD)$(GREEN)📦 Installing xcargo...$(RESET)"
	cargo install --path .
	@echo "$(GREEN)✅ xcargo installed!$(RESET)"

.PHONY: bench
bench: ## ⚡ Run benchmarks
	@echo "$(BOLD)$(YELLOW)⚡ Running benchmarks...$(RESET)"
	cargo bench

# ========================================
# 📚 Documentation Commands
# ========================================

.PHONY: docs-install
docs-install: ## 📥 Install documentation dependencies
	@echo "$(BOLD)$(BLUE)📥 Installing documentation dependencies...$(RESET)"
	cd docs && npm install
	@echo "$(GREEN)✅ Dependencies installed!$(RESET)"

.PHONY: docs-dev
docs-dev: ## 🌐 Start documentation dev server
	@echo "$(BOLD)$(CYAN)🌐 Starting documentation server...$(RESET)"
	cd docs && npm start

.PHONY: docs-build
docs-build: ## 🏗️  Build documentation
	@echo "$(BOLD)$(BLUE)🏗️  Building documentation...$(RESET)"
	cd docs && npm run build
	@echo "$(GREEN)✅ Documentation built!$(RESET)"

.PHONY: docs-serve
docs-serve: ## 🎭 Serve built documentation
	@echo "$(BOLD)$(MAGENTA)🎭 Serving documentation...$(RESET)"
	cd docs && npm run serve

.PHONY: docs-deploy
docs-deploy: ## 🚀 Deploy documentation to GitHub Pages
	@echo "$(BOLD)$(GREEN)🚀 Deploying documentation...$(RESET)"
	cd docs && npm run deploy
	@echo "$(GREEN)✅ Documentation deployed!$(RESET)"

.PHONY: docs-clean
docs-clean: ## 🧹 Clean documentation build
	@echo "$(BOLD)$(RED)🧹 Cleaning documentation...$(RESET)"
	rm -rf docs/build docs/.docusaurus docs/.cache-loader
	@echo "$(GREEN)✅ Documentation cleaned!$(RESET)"

# ========================================
# 🔧 Development Commands
# ========================================

.PHONY: dev
dev: fmt clippy test ## 🔧 Run all development checks
	@echo "$(BOLD)$(GREEN)✅ All development checks passed!$(RESET)"

.PHONY: ci
ci: fmt-check clippy test ## 🤖 Run CI checks
	@echo "$(BOLD)$(GREEN)✅ CI checks passed!$(RESET)"

.PHONY: watch
watch: ## 👀 Watch for changes and run tests
	@echo "$(BOLD)$(YELLOW)👀 Watching for changes...$(RESET)"
	cargo watch -x test

.PHONY: coverage
coverage: ## 📊 Generate code coverage report
	@echo "$(BOLD)$(CYAN)📊 Generating coverage report...$(RESET)"
	cargo tarpaulin --out Html --output-dir coverage
	@echo "$(GREEN)✅ Coverage report generated in coverage/$(RESET)"

# ========================================
# 📦 Release Commands
# ========================================

.PHONY: pre-release
pre-release: ci build-release docs-build ## 📋 Pre-release checklist
	@echo "$(BOLD)$(GREEN)✅ Pre-release checks complete!$(RESET)"
	@echo "$(YELLOW)Ready to publish!$(RESET)"

.PHONY: publish
publish: ## 🎉 Publish to crates.io
	@echo "$(BOLD)$(RED)⚠️  Publishing to crates.io...$(RESET)"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		cargo publish; \
		echo "$(GREEN)✅ Published!$(RESET)"; \
	else \
		echo "$(YELLOW)Cancelled.$(RESET)"; \
	fi

# ========================================
# 🎯 All-in-one Commands
# ========================================

.PHONY: all
all: build test docs-build ## 🎯 Build everything
	@echo "$(BOLD)$(GREEN)✅ Full build complete!$(RESET)"

.PHONY: clean-all
clean-all: clean docs-clean ## 🧹 Clean everything
	@echo "$(BOLD)$(GREEN)✅ Everything cleaned!$(RESET)"

.PHONY: setup
setup: ## 🎬 Initial project setup
	@echo "$(BOLD)$(CYAN)🎬 Setting up xcargo development environment...$(RESET)"
	@echo "$(YELLOW)Installing Rust dependencies...$(RESET)"
	rustup component add clippy rustfmt
	@echo "$(YELLOW)Installing cargo tools...$(RESET)"
	cargo install cargo-watch 2>/dev/null || true
	cargo install cargo-tarpaulin 2>/dev/null || true
	@echo "$(YELLOW)Installing documentation dependencies...$(RESET)"
	cd docs && npm install
	@echo "$(BOLD)$(GREEN)✅ Setup complete!$(RESET)"
	@echo ""
	@echo "$(BOLD)Next steps:$(RESET)"
	@echo "  • Run $(CYAN)make dev$(RESET) to check code"
	@echo "  • Run $(CYAN)make docs-dev$(RESET) to start documentation server"
	@echo "  • Run $(CYAN)make help$(RESET) to see all commands"

# Default target
.DEFAULT_GOAL := help
