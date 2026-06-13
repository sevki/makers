#!/bin/sh
# Run cargo-crap with real coverage data.
#
# Generates an LCOV coverage file via cargo-llvm-cov, then scores every
# function with the CRAP (Change Risk Anti-Patterns) metric. Without
# coverage, cargo-crap treats every function as 0% covered, so this script
# is the meaningful way to run it.
#
# Usage: ./scripts/crap.sh [extra cargo-crap args...]
#   ./scripts/crap.sh --top 20
#   ./scripts/crap.sh --baseline .cargo-crap-baseline.json   # show deltas
#   ./scripts/crap.sh --format json --output .cargo-crap-baseline.json  # refresh baseline
#
# Requires: cargo-crap, cargo-llvm-cov (+ llvm-tools-preview component).
#   cargo install cargo-crap cargo-llvm-cov
#   rustup component add llvm-tools-preview
set -eu

cd "$(dirname "$0")/.."

LCOV="${LCOV:-target/crap-lcov.info}"

if ! cargo llvm-cov --version >/dev/null 2>&1; then
    echo "cargo-llvm-cov not found. Install it with:" >&2
    echo "  cargo install cargo-llvm-cov && rustup component add llvm-tools-preview" >&2
    echo "Falling back to a complexity-only run (every function scored as 0% covered)." >&2
    exec cargo crap "$@"
fi

echo "Collecting coverage into $LCOV ..." >&2
# --ignore-run-fail: still emit coverage even if some tests fail, so a
# flaky/failing test doesn't block the complexity-vs-coverage report.
cargo llvm-cov --ignore-run-fail --lcov --output-path "$LCOV"

echo "Scoring CRAP ..." >&2
exec cargo crap --lcov "$LCOV" "$@"
