

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)


EXECUTABLE = target/debug/crates
RUST_SRC_FILES = $(shell find src -name "*.rs")
bare_index_path = tests/fixtures/index-bare

$(bare_index_path):
	mkdir -p $(dir $@)
	git clone --bare https://github.com/rust-lang/crates.io-index $@

$(EXECUTABLE): $(RUST_SRC_FILES)
	cargo build

##@ Testing

test: bin/test-cli.sh $(bare_index_path) $(EXECUTABLE) ## Run all tests with a bare clone of the crate-io index
	$< $(EXECUTABLE) $(bare_index_path)
