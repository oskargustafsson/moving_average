#!/bin/bash

set -uexo pipefail

rm -rf test_coverage tmp_test_data
mkdir test_coverage
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="tmp_test_data/simple_moving_average-%p-%m.profraw"
cargo test
grcov \
	./tmp_test_data \
	--source-dir . \
	--binary-path ./target/debug/ \
	--output-types html \
	--branch \
	--ignore-not-existing \
	--output-path ./test_coverage/
ls tmp_test_data
rm -rf tmp_test_data
