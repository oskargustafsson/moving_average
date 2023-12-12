build: readme test_coverage

install:
	cargo install cargo-readme
	cargo install grcov
	rustup component add llvm-tools-preview

readme:
	cargo readme > README.md

test_coverage:
	./test_coverage.sh

.PHONY: install readme test_coverage clean
