#!/bin/bash
echo "Tests"
cargo test
# echo "Tests w/ Coverage"
# cargo tarpaulin --ignore-tests
echo "Lint"
cargo clippy -- -D warnings
echo "Format"
cargo fmt -- --check
echo "Audit"
cargo audit
# open ../coverage/tarpaulin-report.html
