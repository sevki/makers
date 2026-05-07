#!/usr/bin/env bash
# Build & install the Rust port of GNU make, then use it to (a) compile the C
# version of GNU make and (b) compile the Linux kernel via okLinux build.sh.
#
# Usage:
#   scripts/ci-test.sh                # run all stages
#   scripts/ci-test.sh --skip-kernel  # skip the kernel build (default for local dev)
#   scripts/ci-test.sh --skip-c-make  # skip the C make stage
#
# Designed to run identically locally and inside .github/workflows/ci.yml.

set -euo pipefail

SKIP_KERNEL=0
SKIP_C_MAKE=0
for arg in "$@"; do
  case "$arg" in
    --skip-kernel)  SKIP_KERNEL=1 ;;
    --skip-c-make)  SKIP_C_MAKE=1 ;;
    -h|--help)
      sed -n '2,11p' "$0"; exit 0 ;;
    *) echo "unknown arg: $arg" >&2; exit 2 ;;
  esac
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

group() { printf '\n=== %s ===\n' "$*"; }

INSTALL_ROOT="$REPO_ROOT/target/install"
INSTALL_BIN="$INSTALL_ROOT/bin"

# ---------------------------------------------------------------------------
# Stage 0: Bootstrap lib/libgnu.a using sh+cc only, via the repo's build.sh.
#
# The rust port's build.rs links against lib/libgnu.a (gnulib). That archive is
# normally produced by the C make build, which creates a chicken-and-egg loop.
# build.sh bootstraps the whole tree using only sh+cc (no make required), so
# we run it solely to produce lib/libgnu.a. The C `make` binary it leaves in
# the repo root is sacrificial and gets overwritten in stage 3.
# ---------------------------------------------------------------------------
group "Stage 0: bootstrap lib/libgnu.a via ./build.sh (sh+cc only)"
if [ ! -f config.status ] || [ ! -f build.cfg ]; then
  ./configure --quiet
fi
./build.sh
test -f lib/libgnu.a || { echo "build.sh did not produce lib/libgnu.a" >&2; exit 1; }

# ---------------------------------------------------------------------------
# Stage 1: Build & install the Rust port of make via `cargo install`.
# ---------------------------------------------------------------------------
group "Stage 1: cargo install --path . (rust make)"
cargo install --path . --bin make --root "$INSTALL_ROOT" --locked --force
test -x "$INSTALL_BIN/make" || {
  echo "rust make not installed at $INSTALL_BIN/make" >&2; exit 1; }
"$INSTALL_BIN/make" --version | head -2

# ---------------------------------------------------------------------------
# Stage 2: Put the installed rust make first on PATH for downstream stages.
# ---------------------------------------------------------------------------
group "Stage 2: prepend installed rust make on PATH"
export PATH="$INSTALL_BIN:$PATH"
hash -r
echo "make resolves to: $(command -v make)"
make --version | head -1

# ---------------------------------------------------------------------------
# Stage 3: Use rust make to compile the C version of GNU make.
# ---------------------------------------------------------------------------
if [ "$SKIP_C_MAKE" -eq 1 ]; then
  echo "Stage 3 skipped (--skip-c-make)"
else
  group "Stage 3: build C make using rust make"
  if [ ! -f Makefile ] || [ ! -f config.status ]; then
    ./configure --quiet
  fi
  # Discard the sacrificial C make binary build.sh produced in stage 0 so the
  # rust make actually has work to do.
  rm -f make
  # `make clean` recurses into lib/ and removes libgnu.a too — that's wanted:
  # the next `make` will rebuild gnulib and the make binary from scratch using
  # the rust make.
  make clean MAKE_CFLAGS= >/dev/null
  # MAKE_CFLAGS in the configured Makefile starts with `-C` (gcc-only "preserve
  # comments" flag). clang rejects it; neutralise it for the build. This is a
  # make-repo build-config quirk, unrelated to rust make.
  make -j"$(nproc)" MAKE_CFLAGS=
  test -x ./make
  C_MAKE_VERSION="$(./make --version | head -1)"
  echo "Built C make: $C_MAKE_VERSION"

  # Smoke test: run the freshly-built C make on a trivial Makefile.
  TMP_MK="$(mktemp -d)/Makefile"
  cat >"$TMP_MK" <<'EOF'
all: ; @echo "C make built by rust make works"
EOF
  ./make -f "$TMP_MK"
fi

# ---------------------------------------------------------------------------
# Stage 4: Use rust make to build the Linux kernel via okLinux build.sh.
# ---------------------------------------------------------------------------
if [ "$SKIP_KERNEL" -eq 1 ]; then
  echo "Stage 4 skipped (--skip-kernel)"
  echo "All requested stages passed."
  exit 0
fi

group "Stage 4: build Linux kernel using rust make (okLinux build.sh)"
KERNEL_WORK="$REPO_ROOT/target/oklinux"
mkdir -p "$KERNEL_WORK"
cd "$KERNEL_WORK"
curl -fsSL \
  https://raw.githubusercontent.com/sevki/okLinux/refs/heads/main/build.sh \
  -o build.sh
chmod +x build.sh
# `make` on PATH is the installed rust make from stage 1.
./build.sh

echo "All stages passed."
