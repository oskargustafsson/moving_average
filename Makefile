build: readme test_coverage

install:
	cargo install cargo-readme
	cargo install grcov
	rustup component add llvm-tools-preview

readme:
	cargo readme > README.md

test_coverage:
	pwd
	rm -rf test_coverage/
	mkdir test_coverage
	export RUSTC_BOOTSTRAP=1
	export RUSTFLAGS="-Zinstrument-coverage"
	export LLVM_PROFILE_FILE="tmp_test_data/simple_moving_average-%p-%m.profraw"
	cargo build
	echo ${LLVM_PROFILE_FILE}
	cargo test
	ls -la
	grcov ./tmp_test_data -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./test_coverage/
	sed -i '/Date:/d' ./test_coverage/*.html ./test_coverage/**/*.html
	rm -r tmp_test_data

.PHONY: install readme test_coverage clean
