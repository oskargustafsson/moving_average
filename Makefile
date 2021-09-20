build: readme test_coverage

install:
	cargo install cargo-readme
	cargo install grcov
	rustup component add llvm-tools-preview

readme:
	cargo readme > README.md

test_coverage:
	rm -rf test_coverage/
	mkdir test_coverage
	export RUSTC_BOOTSTRAP=1
	export RUSTFLAGS="-Zinstrument-coverage"
	export LLVM_PROFILE_FILE="tmp_test_data/simple_moving_average-%p-%m.profraw"
	cargo build
	cargo test
	grcov ./tmp_test_data -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./test_coverage/

.PHONY: install readme test_coverage clean
