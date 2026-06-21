#!/bin/sh
# Print the make-sys crate's OWN unsafe count, as measured by cargo-geiger.
#
# This is the metric the cargo-geiger DIFF GATE compares against the trusted
# main-branch baseline (.cargo-geiger-baseline). It deliberately counts only
# the `make-sys` package (not its dependency tree): as a C2Rust port the crate
# is unsafe-heavy by design, so we gate on REGRESSION of the crate's own
# unsafe footprint, never on an absolute number.
#
# Usage:
#   ./scripts/geiger-unsafe-count.sh [OUT_JSON]
#     OUT_JSON  optional path for the raw geiger JSON report (default: temp file)
#
# Refresh the committed baseline with:
#   ./scripts/geiger-unsafe-count.sh > .cargo-geiger-baseline
#
# Requires: cargo-geiger, python3.
#   cargo install cargo-geiger --locked
set -eu

cd "$(dirname "$0")/.."

OUT="${1:-$(mktemp -t geiger-XXXXXX.json)}"

# cargo-geiger exits non-zero whenever unsafe usage is present, which is the
# normal case for this crate. That is expected; never treat geiger's own exit
# code as a failure. We only care about the JSON it writes.
cargo geiger -p make-sys --all-features --output-format Json > "$OUT" 2>/dev/null || true

# Parse the geiger-serde JSON and sum every integer stored under a key named
# exactly `unsafe_` within the make-sys package's `unsafety` object. That key
# appears in both `used` and `unused`, across all five categories (functions,
# exprs, item_impls, item_traits, methods), so a recursive sum captures the
# crate's total own-unsafe count. Fail loudly (non-zero) if make-sys is not
# found or the JSON cannot be parsed, so the gate never silently passes.
python3 - "$OUT" <<'PY'
import json, sys

path = sys.argv[1]
try:
    with open(path) as f:
        text = f.read()
except OSError as e:
    sys.exit(f"geiger-unsafe-count: cannot read report {path}: {e}")

# geiger may print a non-JSON preamble before the report; start at the first '{'.
start = text.find("{")
if start < 0:
    sys.exit("geiger-unsafe-count: no JSON object found in geiger report")
try:
    data = json.loads(text[start:])
except json.JSONDecodeError as e:
    sys.exit(f"geiger-unsafe-count: failed to parse geiger JSON: {e}")

def find_pkg(data):
    for pkg in data.get("packages", []):
        name = (((pkg.get("package") or {}).get("id") or {}).get("name"))
        if name == "make-sys":
            return pkg
    return None

pkg = find_pkg(data)
if pkg is None:
    sys.exit("geiger-unsafe-count: make-sys package not found in geiger report")

def sum_unsafe(obj):
    total = 0
    if isinstance(obj, dict):
        for k, v in obj.items():
            if k == "unsafe_" and isinstance(v, int):
                total += v
            else:
                total += sum_unsafe(v)
    elif isinstance(obj, list):
        for v in obj:
            total += sum_unsafe(v)
    return total

unsafety = pkg.get("unsafety")
if not isinstance(unsafety, dict):
    sys.exit("geiger-unsafe-count: make-sys package has no unsafety data")

print(sum_unsafe(unsafety))
PY
