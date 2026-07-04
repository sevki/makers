#!/usr/bin/env bash
# Companion to scripts/run-fixtures.sh: handles every manifest row whose
# `kind` isn't "simple" -- cases that don't fit the generic "fixture file
# run through -f/target/args" shape (custom directory layout, signal
# delivery, archive pre-creation, or that must run WITHOUT the
# --no-print-directory flag run-fixtures.sh always passes).
#
# Usage: run-bespoke-fixtures.sh <make-bin> <output-dir>
#
# Writes the same per-fixture layout as run-fixtures.sh (stdout.log,
# stderr.log, exit.txt, tree/) under $OUT_DIR/<name>, so
# scripts/fixtures-diff.sh compares both scripts' output identically. Run
# this against the SAME $OUT_DIR as run-fixtures.sh to cover the full
# manifest.
#
# Manifest columns (tab-separated): name, mode, fixture, target, args, skip, kind
# Column reuse per kind (fixture/target/args have no fixed meaning here;
# each kind repurposes them):
#   print_dir  args   = full argv to run make with (no -f, no forced
#                        --no-print-directory -- that's the whole point).
#                        stdout/stderr are normalized (workdir path -> <WORK>)
#                        before being written to *.log, so the generic
#                        byte-for-byte diff in fixtures-diff.sh just works.
#   dash_i     target = "found" or "notfound" (which -I scenario to run).
#   signal     target = signal name (INT/TERM) to send once the recipe's
#                        leading `touch` proves it's running.
#   ar_glob    (no params -- single fixed case)
set -euo pipefail

MAKE_BIN="${1:?usage: run-bespoke-fixtures.sh <make-bin> <output-dir>}"
OUT_DIR="${2:?usage: run-bespoke-fixtures.sh <make-bin> <output-dir>}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$REPO_ROOT/scripts/fixtures-manifest.tsv"

MAKE_BIN="$(cd "$(dirname "$MAKE_BIN")" && pwd)/$(basename "$MAKE_BIN")"
mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"
# Bash's `read` collapses consecutive tabs (treated as "IFS whitespace" even
# when IFS is set to exactly $'\t'), silently merging empty columns.
# Translate to a non-whitespace byte first so empty fields survive the split.
FIELD_SEP=$'\x01'

# Normalize a captured output file in place: replace both the raw and
# symlink-resolved form of $workdir with a stable placeholder, so runs in
# different tempdirs (different jobs, different machines) compare equal.
normalize_workdir() {
    local file="$1" workdir="$2" resolved
    resolved="$(cd "$workdir" && pwd -P)"
    sed -i -e "s#${resolved//#/\\#}#<WORK>#g" -e "s#${workdir//#/\\#}#<WORK>#g" "$file"
}

snapshot_tree() {
    local src="$1" dest="$2"
    mkdir -p "$dest"
    if [ -n "$(ls -A "$src" 2>/dev/null)" ]; then
        cp -a "$src"/. "$dest/"
    fi
}

run_print_dir() {
    local name="$1" args="$2" fixture_dir="$OUT_DIR/$1"
    mkdir -p "$fixture_dir"
    local workdir; workdir="$(mktemp -d)"
    mkdir -p "$workdir/sub"
    printf 'x:\n\t@echo in-sub\n' >"$workdir/sub/Makefile"

    local argv=()
    IFS=$'\x1f' read -r -a argv <<<"$args"

    set +e
    ( cd "$workdir" && "$MAKE_BIN" "${argv[@]}" >"$fixture_dir/stdout.log" 2>"$fixture_dir/stderr.log" )
    echo $? >"$fixture_dir/exit.txt"
    set -e

    normalize_workdir "$fixture_dir/stdout.log" "$workdir"
    normalize_workdir "$fixture_dir/stderr.log" "$workdir"
    snapshot_tree "$workdir" "$fixture_dir/tree"
    rm -rf "$workdir"
}

run_dash_i() {
    local name="$1" scenario="$2" fixture_dir="$OUT_DIR/$1"
    mkdir -p "$fixture_dir"
    local base; base="$(mktemp -d)"
    mkdir -p "$base/incs" "$base/empty"
    printf 'FROM_INCLUDE := yes\n' >"$base/incs/extra.mk"
    printf 'include extra.mk\nall:\n\t@echo got=$(FROM_INCLUDE)\n' >"$base/Makefile"

    local incdir
    if [ "$scenario" = "found" ]; then incdir="$base/incs"; else incdir="$base/empty"; fi

    set +e
    ( cd "$base" && "$MAKE_BIN" --no-print-directory -I "$incdir" -f "$base/Makefile" all \
        >"$fixture_dir/stdout.log" 2>"$fixture_dir/stderr.log" )
    echo $? >"$fixture_dir/exit.txt"
    set -e

    normalize_workdir "$fixture_dir/stdout.log" "$base"
    normalize_workdir "$fixture_dir/stderr.log" "$base"
    snapshot_tree "$base" "$fixture_dir/tree"
    rm -rf "$base"
}

run_signal() {
    local name="$1" sig="$2" fixture_dir="$OUT_DIR/$1"
    mkdir -p "$fixture_dir"
    local workdir; workdir="$(mktemp -d)"
    printf 'slow: ; @touch slow && sleep 5\n' >"$workdir/Makefile"

    ( cd "$workdir" && exec "$MAKE_BIN" --no-print-directory ) \
        >"$fixture_dir/stdout.log" 2>"$fixture_dir/stderr.log" &
    local pid=$!

    local target="$workdir/slow" waited=0
    while [ ! -e "$target" ] && [ "$waited" -lt 5000 ]; do
        sleep 0.01
        waited=$((waited + 10))
    done
    if [ ! -e "$target" ]; then
        echo "::error::[$name] recipe never started" >&2
    fi

    kill "-$sig" "$pid"
    set +e
    wait "$pid"
    echo $? >"$fixture_dir/exit.txt"
    set -e

    normalize_workdir "$fixture_dir/stdout.log" "$workdir"
    normalize_workdir "$fixture_dir/stderr.log" "$workdir"
    snapshot_tree "$workdir" "$fixture_dir/tree"
    rm -rf "$workdir"
}

run_ar_glob() {
    local name="$1" fixture_dir="$OUT_DIR/$1"
    mkdir -p "$fixture_dir"
    local dir; dir="$(mktemp -d)"
    local members=(zeta.o alpha.o Mid.o beta.o mid.o)
    for m in "${members[@]}"; do printf 'x\n' >"$dir/$m"; done
    ( cd "$dir" && ar rc libdiff.a "${members[@]}" ) >/dev/null
    printf "all: ; @echo '[\$(wildcard libdiff.a(*.o))]'\n" >"$dir/Makefile"

    set +e
    ( cd "$dir" && "$MAKE_BIN" --no-print-directory all \
        >"$fixture_dir/stdout.log" 2>"$fixture_dir/stderr.log" )
    echo $? >"$fixture_dir/exit.txt"
    set -e

    snapshot_tree "$dir" "$fixture_dir/tree"
    rm -rf "$dir"
}

count=0
while IFS="$FIELD_SEP" read -r name mode fixture target args skip kind; do
    [ -n "$name" ] || continue
    case "$kind" in
        simple) continue ;;
        print_dir) run_print_dir "$name" "$args" ;;
        dash_i) run_dash_i "$name" "$target" ;;
        signal) run_signal "$name" "$target" ;;
        ar_glob) run_ar_glob "$name" ;;
        *)
            echo "::error::unknown fixture kind '$kind' for '$name'" >&2
            exit 1
            ;;
    esac
    count=$((count + 1))
    printf '  %s: exit=%s\n' "$name" "$(cat "$OUT_DIR/$name/exit.txt")"
done < <(tail -n +2 "$MANIFEST" | tr '\t' "$FIELD_SEP")

echo "Ran $count bespoke fixtures with $MAKE_BIN"
echo "Wrote fixture outputs to $OUT_DIR"
