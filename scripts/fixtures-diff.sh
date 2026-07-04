#!/usr/bin/env bash
# Compare the fixture outputs produced by two independent `run-fixtures.sh`
# runs (one driven by the C make oracle, one by the Rust make) for
# divergence. Used by the fixtures-diff CI job, which downloads both
# artifacts uploaded by the fixtures-run-rust / fixtures-run-c jobs.
#
# Usage: fixtures-diff.sh <c-dir> <rust-dir>
#
# For each fixture in scripts/fixtures-manifest.tsv:
#   - exit code must match exactly.
#   - stdout (and, unless mode is unordered_stdout_only, stderr) must match:
#     byte-for-byte for `ordered` fixtures, as a sorted line multiset for
#     `unordered`/`unordered_stdout_only` fixtures (make's own recipe
#     interleaving under -j is not stable — see tests/rs_integration.rs).
#   - the working tree left behind must match: same paths, same content,
#     same permission bits. mtimes are deliberately never compared (the two
#     runs happen at different wall-clock times and possibly different
#     machines) -- this is the "ignore time differences" gate.
# Fixtures with a non-empty `skip` column (quarantined known divergences,
# tracked by issue) are reported but do not fail the job.
#
# On divergence, diffoscope renders the human-readable report (install
# diffoscope-minimal; CI has it) into fixtures-<name>.diff.
set -uo pipefail

C_DIR="${1:?usage: fixtures-diff.sh <c-dir> <rust-dir>}"
RUST_DIR="${2:?usage: fixtures-diff.sh <c-dir> <rust-dir>}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$REPO_ROOT/scripts/fixtures-manifest.tsv"
SEP=$'\x1f'
# Bash's `read` treats tab as "IFS whitespace" even when IFS is set to
# exactly $'\t' -- consecutive delimiters collapse and leading/trailing ones
# are stripped, silently merging empty columns (e.g. args="", skip="").
# Translate tabs to a non-whitespace byte first so every column survives.
FIELD_SEP=$'\x01'

fail=0
skipped=0
checked=0

# Flat, sorted "path mode filetype" listing for permission/structure
# comparison -- deliberately excludes mtime. `find -printf` isn't portable
# (no macOS), so this shells out per-entry via `stat`; fixture trees are tiny
# so this is not a performance concern.
tree_modes() {
    local root="$1"
    [ -d "$root" ] || return 0
    (cd "$root" && find . -mindepth 1 | sort | while read -r p; do
        stat -c '%n %a %F' "$p" 2>/dev/null || stat -f '%N %Lp %HT' "$p"
    done)
}

while IFS="$FIELD_SEP" read -r name mode fixture target args skip kind; do
    [ -n "$name" ] || continue
    if [ -n "$skip" ]; then
        echo "SKIP  $name ($skip)"
        skipped=$((skipped + 1))
        continue
    fi
    checked=$((checked + 1))

    c_fx="$C_DIR/$name"
    r_fx="$RUST_DIR/$name"
    if [ ! -d "$c_fx" ] || [ ! -d "$r_fx" ]; then
        echo "::error::fixture '$name' missing from an artifact (C: $c_fx, Rust: $r_fx)"
        fail=1
        continue
    fi

    cmp_c="$(mktemp -d)"
    cmp_r="$(mktemp -d)"

    cp "$c_fx/exit.txt" "$cmp_c/exit.txt"
    cp "$r_fx/exit.txt" "$cmp_r/exit.txt"

    case "$mode" in
        unordered | unordered_stdout_only)
            sort "$c_fx/stdout.log" >"$cmp_c/stdout.log"
            sort "$r_fx/stdout.log" >"$cmp_r/stdout.log"
            ;;
        *)
            cp "$c_fx/stdout.log" "$cmp_c/stdout.log"
            cp "$r_fx/stdout.log" "$cmp_r/stdout.log"
            ;;
    esac

    if [ "$mode" = "unordered" ]; then
        sort "$c_fx/stderr.log" >"$cmp_c/stderr.log"
        sort "$r_fx/stderr.log" >"$cmp_r/stderr.log"
    elif [ "$mode" != "unordered_stdout_only" ]; then
        cp "$c_fx/stderr.log" "$cmp_c/stderr.log"
        cp "$r_fx/stderr.log" "$cmp_r/stderr.log"
    fi
    # unordered_stdout_only: stderr intentionally excluded from the
    # comparison view (mirrors the original in-process test, which never
    # compared stderr for these two fixtures).

    mkdir -p "$cmp_c/tree" "$cmp_r/tree"
    [ -d "$c_fx/tree" ] && [ -n "$(ls -A "$c_fx/tree" 2>/dev/null)" ] && cp -a "$c_fx/tree/." "$cmp_c/tree/"
    [ -d "$r_fx/tree" ] && [ -n "$(ls -A "$r_fx/tree" 2>/dev/null)" ] && cp -a "$r_fx/tree/." "$cmp_r/tree/"

    diverged=0
    if ! diff -rq "$cmp_c" "$cmp_r" >/dev/null 2>&1; then
        diverged=1
    fi
    if ! diff <(tree_modes "$cmp_c/tree") <(tree_modes "$cmp_r/tree") >/dev/null 2>&1; then
        diverged=1
    fi

    if [ "$diverged" -eq 1 ]; then
        echo "::error::fixture '$name' diverged between C make and Rust make"
        if command -v diffoscope >/dev/null 2>&1; then
            diffoscope --no-progress --exclude-directory-metadata=recursive \
                "$cmp_c" "$cmp_r" >"fixtures-${name}.diff" 2>&1 || true
            echo "----- diffoscope report: fixtures-${name}.diff -----"
            head -100 "fixtures-${name}.diff"
        fi
        fail=1
    else
        echo "OK    $name"
    fi

    rm -rf "$cmp_c" "$cmp_r"
done < <(tail -n +2 "$MANIFEST" | tr '\t' "$FIELD_SEP")

echo ""
echo "fixtures-diff: $checked checked, $skipped skipped (known divergence)"
if [ "$fail" -ne 0 ]; then
    echo "::error::one or more fixtures diverged between C make and Rust make"
fi
exit "$fail"
