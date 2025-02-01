# Makefile for building and installing RASM

# Set the binary name and path to release build
BINARY_NAME = rasm
BUILD_DIR = target/release
BINARY_PATH = $(BUILD_DIR)/$(BINARY_NAME)
INSTALL_DIR = /usr/local/bin

.PHONY: all build install alias clean

# Default target builds the binary
all: build

# Build the project in release mode
build:
	cargo build --release

# Install target: builds the binary, copies it to /usr/local/bin,
# and creates an alias in the user's shell configuration.
install: build
	@echo "Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	# Copy the binary to /usr/local/bin (may require sudo)
	sudo cp $(BINARY_PATH) $(INSTALL_DIR)/$(BINARY_NAME)
	@$(MAKE) alias

# The alias target will detect the shell configuration file and append an alias.
alias:
	@echo "Detecting shell configuration file..."
	@if [ -n "$$ZSH_VERSION" ]; then \
	    SHELL_CONFIG=$$HOME/.zshrc; \
	elif [ -n "$$BASH_VERSION" ]; then \
	    SHELL_CONFIG=$$HOME/.bashrc; \
	else \
	    SHELL_CONFIG=$$HOME/.profile; \
	fi; \
	echo "Using shell config file: $$SHELL_CONFIG"; \
	if ! grep -q "^alias $(BINARY_NAME)=" $$SHELL_CONFIG; then \
	    echo "alias $(BINARY_NAME)='$(INSTALL_DIR)/$(BINARY_NAME)'" >> $$SHELL_CONFIG; \
	    echo "Alias added to $$SHELL_CONFIG. Reload your shell or run 'source $$SHELL_CONFIG' to use it."; \
	else \
	    echo "Alias for $(BINARY_NAME) already exists in $$SHELL_CONFIG"; \
	fi

# Clean the project (using cargo clean)
clean:
	cargo clean
