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

# Normalize a build log down to make-relevant lines so the two makes' outputs
# are comparable:
#   - okLinux build.sh first runs `git clone` and `sudo apt-get update/install`,
#     whose output (fetch timings, mirror chatter, package-manager messages) is
#     run-specific and unrelated to make. So we *allowlist* only kbuild's recipe
#     echoes ("  CC  foo.o", "  LD  vmlinux", ...) — an UPPERCASE verb at line
#     start — and make's own program/status lines ("make:" / "make[N]:"),
#     dropping everything else.
#   - strip the per-build working-directory prefixes (the two builds run in
#     different temp dirs), and drop make's directory-recursion chatter.
#   - sort to tolerate `make -j` non-deterministic interleaving, but DO NOT
#     dedupe: multiplicity matters — a make that runs a recipe twice, or skips
#     one of two identical actions, is exactly the kind of regression this is
#     meant to catch.
# Tunable: widen the path stripping / allowlist if real runs show spurious diffs.
normalize() {
    sed -E \
        -e 's#/[^[:space:]]*/(rust|c)-kernel/##g' \
        -e 's#/(home|tmp|mnt|runner)/[^[:space:]]*/kernel/#kernel/#g' \
        "$1" 2>/dev/null \
        | grep -E '^[[:space:]]*([A-Z][A-Z0-9_]+([[:space:]]|$)|make(\[[0-9]+\])?:)' \
        | grep -vE "make\[[0-9]+\]: (Entering|Leaving) directory" \
        | sed -E 's/^[[:space:]]+//' \
        | sort
}

for stream in stdout stderr; do
    echo "=== ${stream}: C make vs Rust make (after normalization) ==="
    c_norm="$(mktemp)"
    r_norm="$(mktemp)"
    normalize "$c_dir/kernel-${stream}.log" >"$c_norm"
    normalize "$rust_dir/kernel-${stream}.log" >"$r_norm"

    # diffoscope renders the human-readable report (install diffoscope-minimal
    # locally for it; CI has it) and its exit code is meaningful (0 = no
    # differences, 1 = differences found); fall back to plain diff if it's
    # unavailable.
    if command -v diffoscope >/dev/null 2>&1; then
        diffoscope --no-progress "$c_norm" "$r_norm" >"${stream}.diff"
        rc=$?
    else
        diff -u "$c_norm" "$r_norm" >"${stream}.diff"
        rc=$?
    fi
    rm -f "$c_norm" "$r_norm"

    if [ "$rc" -eq 0 ]; then
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
