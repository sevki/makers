# Coverage delta for c2rust cleanup passes

[AGENTS.md](AGENTS.md) requires every cleanup pass to **raise or preserve
coverage**: the `cargo-llvm-cov` line-coverage delta against the base branch
must be `>= 0`, and any code a pass touches needs a `#[cfg(test)]` unit test
and/or a fixture in `scripts/fixtures-manifest.tsv` (plus its matching
`tests/rs_integration.rs` case) that checks `make` behavior byte-for-byte
against the in-tree C oracle in the `fixtures-diff` CI job.

This document describes how to measure that delta locally before opening a PR.

## The command

```sh
./scripts/coverage-delta.sh                 # report-only vs origin/main
./scripts/coverage-delta.sh --base HEAD~1   # compare against a specific ref
./scripts/coverage-delta.sh --enforce       # exit non-zero on a regression
```

The script runs the instrumented test suite twice — once against the base
(checked out in a throwaway `git worktree`, so your working tree is never
touched) and once against your working tree — then prints `head - base` line
coverage. It scores against the **merge-base** of `HEAD` and the base ref, so
a base branch that has moved ahead does not skew the result.

`--enforce` turns a negative delta into a non-zero exit; this is the gate a PR
author runs before pushing. The default is report-only so the command is safe
to run for information without failing.

## Local setup

Requirements:

```sh
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
# python3 and git are also required (both are already needed by the repo).
```

### No C oracle needed locally

`cargo test` (what `coverage-delta.sh` measures) only smoke-tests the Rust
make itself — it no longer builds or shells out to the C oracle. The
differential comparison against the C oracle runs separately in CI:
`scripts/run-fixtures.sh` drives every fixture in
`scripts/fixtures-manifest.tsv` through one binary at a time (the
`fixtures-run-rust` / `fixtures-run-c` jobs), and `scripts/fixtures-diff.sh`
(the `fixtures-diff` job) diffoscopes the two resulting artifact sets. So
measuring a local coverage delta needs nothing beyond the requirements above.

## CI mode: report-only (for now)

CI runs coverage **report-only**, by deliberate choice:

- The `coverage (cargo-llvm-cov)` job in `.github/workflows/ci.yml` is
  `continue-on-error: true`. It publishes `lcov.info`, prints the summary to
  the job summary, and posts/refreshes a coverage comment on the PR, but a
  coverage number does **not** block the run.
- `cargo-llvm-cov` line coverage has mild run-to-run nondeterminism (recipe
  execution timing, environment-dependent branches), so a hard CI gate on a
  raw delta would be flaky. The committed-baseline **CRAP gate**
  (`scripts/crap.sh` + `.cargo-crap-baseline.json`) already blocks
  coverage-weighted complexity *regressions* against a trusted baseline.

The enforcing gate therefore lives with the PR author locally
(`./scripts/coverage-delta.sh --enforce`). To promote it to a hard CI gate
later, add a job that runs the same script with `--enforce` against the PR
base; the report-only default keeps that change a one-flag flip.

## See also

- [`scripts/crap.sh`](scripts/crap.sh) — coverage-weighted CRAP scoring.
- [AGENTS.md](AGENTS.md) — the "always raise coverage" rule this enforces.
