#!/usr/bin/env bash
# Run every fixture in scripts/fixtures-manifest.tsv through ONE `make` binary,
# capturing stdout/stderr/exit-code and a snapshot of the resulting working
# tree per fixture. Used by the CI fixtures-run-rust / fixtures-run-c jobs so
# the Rust make and the C make each run independently (in parallel CI jobs);
# a later job (scripts/fixtures-diff.sh) diffs the two output directories.
#
# Usage: run-fixtures.sh <make-bin> <output-dir>
#
# Manifest columns (tab-separated): name, mode, fixture, target, args, skip, kind
#   - args is a list of argv entries joined with \x1f (some entries, e.g.
#     `--eval=EV := from_eval`, contain literal spaces, so plain
#     space-joining would misparse them back into argv).
#   - mode/skip are not needed here (they drive comparison, not execution)
#     but are carried through so the two scripts share one manifest.
#   - kind selects the runner: this script only handles "simple" rows (a
#     fixture file run via `-f`/target/args); every other kind needs
#     bespoke setup (custom directory layout, signal delivery, archive
#     pre-creation, no --no-print-directory) and is handled by the
#     companion scripts/run-bespoke-fixtures.sh instead. Run both scripts
#     against the same $OUT_DIR to cover the full manifest.
set -euo pipefail

MAKE_BIN="${1:?usage: run-fixtures.sh <make-bin> <output-dir>}"
OUT_DIR="${2:?usage: run-fixtures.sh <make-bin> <output-dir>}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$REPO_ROOT/scripts/fixtures-manifest.tsv"
FIXTURES_DIR="$REPO_ROOT/tests/fixtures"
SEP=$'\x1f'
# Bash's `read` treats tab as "IFS whitespace" even when IFS is set to
# exactly $'\t' -- consecutive delimiters collapse and leading/trailing ones
# are stripped, silently merging empty columns (e.g. args="", skip="").
# Translate tabs to a non-whitespace byte first so every column, empty or
# not, survives the split.
FIELD_SEP=$'\x01'

MAKE_BIN="$(cd "$(dirname "$MAKE_BIN")" && pwd)/$(basename "$MAKE_BIN")"
mkdir -p "$OUT_DIR"
# Resolve to an absolute path: each fixture's make runs in a tempdir (`cd
# "$workdir"` below), so a relative $OUT_DIR would have its stdout/stderr
# redirects resolve against that tempdir instead of the caller's cwd.
OUT_DIR="$(cd "$OUT_DIR" && pwd)"

echo "Running $(tail -n +2 "$MANIFEST" | awk -F'\t' '$7 == "simple"' | wc -l) fixtures with $MAKE_BIN"

tail -n +2 "$MANIFEST" | tr '\t' "$FIELD_SEP" | while IFS="$FIELD_SEP" read -r name mode fixture target args skip kind; do
    [ -n "$name" ] || continue
    [ "$kind" = "simple" ] || continue
    fixture_dir="$OUT_DIR/$name"
    mkdir -p "$fixture_dir"
    workdir="$(mktemp -d)"

    argv=()
    if [ -n "$args" ]; then
        IFS="$SEP" read -r -a argv <<<"$args"
    fi

    set +e
    (
        cd "$workdir"
        "$MAKE_BIN" --no-print-directory -f "$FIXTURES_DIR/$fixture" "${argv[@]}" "$target" \
            >"$fixture_dir/stdout.log" 2>"$fixture_dir/stderr.log"
    )
    echo $? >"$fixture_dir/exit.txt"
    set -e

    mkdir -p "$fixture_dir/tree"
    # Copy rather than move: leaves $workdir for local debugging, and avoids
    # surprises if two fixtures somehow shared a tempdir.
    if [ -n "$(ls -A "$workdir" 2>/dev/null)" ]; then
        cp -a "$workdir"/. "$fixture_dir/tree/"
    fi
    rm -rf "$workdir"

    printf '  %s: exit=%s\n' "$name" "$(cat "$fixture_dir/exit.txt")"
done

echo "Wrote fixture outputs to $OUT_DIR"
