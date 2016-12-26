info:
	$(info Available Targets)
	$(info ---------------------------------------------------------------------------)
	$(info test       | run all tests)
	
CARGO = $(shell command -v cargo)
EXECUTABLE = target/debug/crates
RUST_SRC_FILES = $(shell find src -name "*.rs")
bare_index_path = tests/fixtures/index-bare

$(bare_index_path):
	mkdir -p $(dir $@)
	git clone --bare https://github.com/rust-lang/crates.io-index $@

$(EXECUTABLE): $(RUST_SRC_FILES)
	$(CARGO) build

test: bin/test-cli.sh $(bare_index_path) $(EXECUTABLE)
	$< $(EXECUTABLE) $(bare_index_path)
