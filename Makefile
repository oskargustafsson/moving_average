build: readme test_coverage

install:
	cargo install cargo-readme
	cargo install grcov
	rustup component add llvm-tools-preview

readme:
	cargo readme > README.md

test_coverage:
	export RUSTC_BOOTSTRAP=1
	export RUSTFLAGS="-Zinstrument-coverage"
	cargo build
	export LLVM_PROFILE_FILE="simple_moving_average-%p-%m.profraw"
	cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./test_coverage/
	rm *.profraw

.PHONY: install readme test_coverage clean
