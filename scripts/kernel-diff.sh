#!/usr/bin/env bash
# Compare two Linux kernel builds — one driven by the Rust make, one by the C
# make — for divergence. Used by the kernel-diff CI job to catch regressions
# where the Rust make drives the build differently from upstream make.
#
# Usage: kernel-diff.sh <c-logdir> <rust-logdir>
# Each <logdir> holds kernel-stdout.log, kernel-stderr.log, kernel-exit.txt
# produced by running okLinux build.sh under the respective make.
set -uo pipefail

c_dir="${1:?usage: kernel-diff.sh <c-logdir> <rust-logdir>}"
rust_dir="${2:?usage: kernel-diff.sh <c-logdir> <rust-logdir>}"
fail=0

c_exit=$(cat "$c_dir/kernel-exit.txt" 2>/dev/null || echo missing)
r_exit=$(cat "$rust_dir/kernel-exit.txt" 2>/dev/null || echo missing)
echo "C make    build exit code: $c_exit"
echo "Rust make build exit code: $r_exit"
[ "$c_exit" = 0 ] || { echo "::error::C make kernel build did not succeed (exit $c_exit)"; fail=1; }
[ "$r_exit" = 0 ] || { echo "::error::Rust make kernel build did not succeed (exit $r_exit)"; fail=1; }

# Normalize a build log so the two makes' outputs are comparable:
#   - strip the per-build working-directory prefixes (the two builds run in
#     different temp dirs),
#   - drop make's own directory-tracking chatter,
#   - trim leading whitespace from kbuild's "  CC  foo.o" echoes,
#   - sort uniquely, since `make -j` interleaves output non-deterministically —
#     so we compare the *set* of build actions/messages, not their order.
# Tunable: widen the path stripping if real runs show spurious diffs.
normalize() {
    sed -E \
        -e 's#/[^[:space:]]*/(rust|c)-kernel/##g' \
        -e 's#/(home|tmp|mnt|runner)/[^[:space:]]*/kernel/#kernel/#g' \
        "$1" 2>/dev/null \
        | grep -vE "make\[[0-9]+\]: (Entering|Leaving) directory" \
        | sed -E 's/^[[:space:]]+//' \
        | sort -u
}

for stream in stdout stderr; do
    echo "=== ${stream}: C make vs Rust make (after normalization) ==="
    if diff -u \
        <(normalize "$c_dir/kernel-${stream}.log") \
        <(normalize "$rust_dir/kernel-${stream}.log") >"${stream}.diff"; then
        echo "${stream}: identical"
    else
        echo "::error::kernel build ${stream} differs between C make and Rust make"
        echo "----- first 200 lines of normalized ${stream} diff (< C, > Rust) -----"
        head -200 "${stream}.diff"
        fail=1
    fi
done

if [ "$fail" -ne 0 ]; then
    echo "::error::Rust make and C make produced divergent kernel builds"
fi
exit "$fail"
