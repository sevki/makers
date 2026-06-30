#!/bin/sh
# Measure the cargo-llvm-cov line-coverage delta between a base ref and the
# current working tree, enforcing AGENTS.md's rule that every c2rust cleanup
# pass keep the coverage delta >= 0.
#
# It runs the instrumented test suite twice — once against the base (in a
# throwaway git worktree, so your checkout is never touched) and once against
# the working tree — and reports `current - base` line coverage. The
# differential tests that compare the Rust port against the in-tree C oracle
# (`./make`) are included automatically when that binary is present; the
# existing oracle is copied into the base worktree so both sides exercise the
# same paths.
#
# Usage:
#   ./scripts/coverage-delta.sh                 # report-only vs origin/main
#   ./scripts/coverage-delta.sh --base HEAD~1   # compare against another ref
#   ./scripts/coverage-delta.sh --enforce       # exit non-zero if delta < 0
#
# Default mode is report-only (always exits 0 unless something errors); pass
# --enforce to turn a coverage regression into a failure, which is how a PR
# author gates a cleanup pass locally before pushing. See doc/coverage.md.
#
# Requires: cargo-llvm-cov (+ llvm-tools-preview), python3, git.
#   cargo install cargo-llvm-cov && rustup component add llvm-tools-preview
set -eu

usage() {
    sed -n '2,30p' "$0" | sed 's/^# \{0,1\}//'
}

BASE=""
ENFORCE=0
while [ $# -gt 0 ]; do
    case "$1" in
        --base) BASE="${2:?--base needs a ref}"; shift 2 ;;
        --base=*) BASE="${1#*=}"; shift ;;
        --enforce) ENFORCE=1; shift ;;
        -h|--help) usage; exit 0 ;;
        --) shift; break ;;
        -*) echo "unknown option: $1" >&2; usage >&2; exit 2 ;;
        *)
            if [ -z "$BASE" ]; then BASE="$1"; shift
            else echo "unexpected argument: $1" >&2; exit 2; fi ;;
    esac
done

cd "$(dirname "$0")/.."
ROOT="$(pwd)"

if ! cargo llvm-cov --version >/dev/null 2>&1; then
    echo "cargo-llvm-cov not found. Install it with:" >&2
    echo "  cargo install cargo-llvm-cov && rustup component add llvm-tools-preview" >&2
    exit 1
fi
if ! command -v python3 >/dev/null 2>&1; then
    echo "python3 is required to parse the coverage JSON." >&2
    exit 1
fi

# Default to origin/main, falling back to a local main when there is no remote.
if [ -z "$BASE" ]; then
    if git rev-parse --verify -q origin/main >/dev/null; then BASE=origin/main; else BASE=main; fi
fi
# Score against the fork point (merge-base), not a base branch that has since
# moved ahead — otherwise unrelated base commits would skew the delta.
BASE_REF="$(git merge-base HEAD "$BASE" 2>/dev/null || git rev-parse --verify "$BASE")"
BASE_SHORT="$(git rev-parse --short "$BASE_REF")"

if [ ! -x "$ROOT/make" ]; then
    echo "note: C oracle ./make not found; the differential tests will be skipped" >&2
    echo "      on both sides. Build it first for a representative delta:" >&2
    echo "        make MAKE_CFLAGS=\"-Wall\"      # or ./build.sh" >&2
fi

# Run the instrumented suite in $1 and print its total line-coverage percent.
measure() {
    _dir="$1"
    _json="$(mktemp)"
    # --ignore-run-fail: still emit coverage if a flaky test fails, matching
    # scripts/crap.sh, so the delta always has data on both sides.
    ( cd "$_dir" && cargo llvm-cov --ignore-run-fail --json --output-path "$_json" >/dev/null )
    python3 - "$_json" <<'PY'
import json, sys
with open(sys.argv[1]) as f:
    d = json.load(f)
print("%.4f" % d["data"][0]["totals"]["lines"]["percent"])
PY
    rm -f "$_json"
}

echo "Measuring base coverage ($BASE -> $BASE_SHORT) in a scratch worktree ..." >&2
WT="$(mktemp -d)"
cleanup() {
    git worktree remove --force "$WT" >/dev/null 2>&1 || true
    rm -rf "$WT" >/dev/null 2>&1 || true
}
trap cleanup EXIT INT TERM
git worktree add --detach "$WT" "$BASE_REF" >/dev/null 2>&1
# Reuse the already-built C oracle so the base side runs the differential tests
# too (the C sources are a differential oracle, identical across the branches).
[ -x "$ROOT/make" ] && cp "$ROOT/make" "$WT/make"
BASE_PCT="$(measure "$WT")"

echo "Measuring working-tree coverage ..." >&2
CUR_PCT="$(measure "$ROOT")"

DELTA="$(awk -v a="$CUR_PCT" -v b="$BASE_PCT" 'BEGIN { printf "%.4f", a - b }')"
DELTA_SIGNED="$(awk -v a="$CUR_PCT" -v b="$BASE_PCT" 'BEGIN { printf "%+.4f", a - b }')"

printf '\n'
printf 'base (%s) line coverage: %s%%\n' "$BASE_SHORT" "$BASE_PCT"
printf 'head           line coverage: %s%%\n' "$CUR_PCT"
printf 'delta                       : %s pts\n' "$DELTA_SIGNED"
printf '\n'

if awk -v d="$DELTA" 'BEGIN { exit (d < 0) ? 0 : 1 }'; then
    echo "Coverage REGRESSED. Add tests for the code this pass touches (AGENTS.md: delta must be >= 0)." >&2
    [ "$ENFORCE" -eq 1 ] && exit 1
    echo "(report-only; re-run with --enforce to fail on regression)" >&2
else
    echo "Coverage delta is >= 0." >&2
fi
