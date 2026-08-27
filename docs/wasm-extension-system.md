# Design: wasm Component Model extension system for make

Tracking issue: [#633](https://github.com/sevki/makers/issues/633).
Related: [#620](https://github.com/sevki/makers/issues/620) (tracking: compile
`make` to wasm), [#628](https://github.com/sevki/makers/issues/628) (PR:
compile `make` to `wasm32-wasip1`).

## Summary

Give `make` a WebAssembly Component Model-based extension mechanism:
extensions are wasm components with WIT-defined interfaces, loaded and
executed by `make`. This is the Bazel/Starlark analog for this project, but
using sandboxed wasm components instead of an embedded scripting language.

This is a different, complementary axis from #620/#628:

- **#620/#628 — make `make` a wasm *guest***: `make` itself compiles to and
  runs inside `wasm32-wasip1` (or in a browser / Cloudflare Worker).
- **#633 (this doc) — make `make` a wasm *host***: `make` embeds or delegates
  to a component-model runtime to load and execute extension components,
  independent of language, and independent of what target `make` itself is
  compiled for.

The two axes compose: `make` must work as a component host whether it is
running natively or running as a wasm guest itself. That composition is the
main design pressure in this doc — see [Two deployment shapes](#two-deployment-shapes-native-vs-wasm-guest).

> **Update.** The interface sketched below as "read-only introspection"
> shipped as `makers:introspection@0.1.0` in
> [#647](https://github.com/sevki/makers/pull/647) and has since been
> redesigned. The current contract is `makers:plugin@1.0.0` — see
> [`docs/plugin-api.md`](plugin-api.md), which supersedes this document's
> "Open questions" on interface shape. The architecture below (two
> deployment shapes, one WIT contract, runtime choice deferred) is unchanged
> and still governs.

## Motivating context

This codebase already has two prior extension mechanisms, both dead ends:

- **`dlopen`-based plugin API** — `src/load.rs`, `src/loadapi.rs`, matching
  upstream GNU Make's `gnumake.h` load API (`gmk_*` functions). Native-only,
  unsandboxed, ABI-fragile across platforms and compilers.
- **Guile scripting integration** — `src/guile.rs`, currently a disabled
  no-op stub.

Neither survives contact with "extensions must run regardless of language"
or "the host itself might be running inside a browser." A `dlopen`'d native
`.so` cannot be loaded from inside a wasm guest at all; Guile is a single
embedded language, not a polyglot extension surface.

## Decisions (from issue discussion)

These were answered directly by @sevki on the issue and are treated as
settled for the MVP:

1. **Scope of first extension capability: read-only introspection.**
   Query targets/prereqs/variables from the build graph. Defining new rules
   or acting as a custom recipe executor (replacing `job.rs`'s shell-out)
   are explicitly deferred — both mean an extension can change what gets
   built or how, which is a much bigger trust and sandboxing surface than
   read-only queries, and the WIT interface can't be validated under real
   use until something is actually consuming it.
2. **Runtime is not fixed to `wasmtime`.** `wasmtime` was the original
   default assumption (mature `wasmtime-wasi` for capability-scoped FS,
   reference component-model implementation), but v8 (`rusty_v8`) was also
   raised as a candidate. See [Runtime choice](#runtime-choice-depends-on-deployment-shape)
   below — the answer is now shape-dependent rather than a single pick.
3. **Replaces `dlopen` and Guile.** No long-term coexistence. Decided and
   executed (#639): the `gmk_*` ABI surface (`src/loadapi.rs`) and the
   disabled Guile stub (`src/guile.rs`) were removed atomically, not
   deprecated — `load_file` (`src/load.rs`) already reported `load` as
   unsupported on every path in this port (dlopen was never actually wired
   up), so nothing reachable from a real Makefile changed, and there is no
   working `gmk_*`-based plugin in the wild to migrate. `src/load.rs` itself
   is kept as-is: it implements the `load` directive/`--load` flag's
   "unsupported on this platform" behavior, which mirrors upstream GNU
   Make's own non-dlopen build configuration and is orthogonal to the
   component-model work — a future decision to make `load` dispatch to wasm
   extensions is out of scope here.
4. **MVP scope, not full-surface-upfront design.** Interface grows from a
   narrow introspection-only start rather than being fully speced before
   any extension exists.

## Two deployment shapes: native vs. wasm guest

The requirement that surfaced last and reshapes the runtime question: `make`
itself needs to run as wasm too — in a browser, or in a Cloudflare Worker
(per #620/#628) — and *in that mode* it still needs to load and call
extensions written in whatever language, sandboxed, via the same conceptual
interface as the native build.

This means "component host" is not a single architecture; it's two, sharing
one WIT contract:

### Shape A — native `make`

`make` is a normal native process. It can embed a component-model runtime
directly and be the real host: load a `.wasm` component file from disk,
instantiate it, satisfy its WASI imports (capability-scoped filesystem via
`wasmtime-wasi`/`cap-std`), and call its exports.

### Shape B — `make` as a wasm guest (browser / CF Worker)

`make` itself is a wasm module running inside another engine (V8 in the
browser, `workerd` in a Cloudflare Worker, or a wasm engine in either).
Wasm does not nest cleanly: `make`'s own wasm instance cannot embed a
second full wasm engine (`wasmtime`-in-wasm, or `rusty_v8`-in-wasm) to then
host *plugin* components — that's both largely unsupported by these
runtimes today and pure round-trip overhead even where it technically
works.

In this shape, `make` cannot be the component host itself. Instead, `make`
becomes a **guest that imports a "load and call a component" capability**,
and the *outer* environment — the browser's JS engine via a component-model
JS glue layer (e.g. `jco`), or the Worker runtime — satisfies that import
and does the actual instantiation/execution of the plugin component. Plugin
loading is delegated outward, not self-hosted, when `make` is itself a
guest.

### Why one WIT contract still works across both shapes

Both shapes present the same logical interface to extension authors and to
the rest of `make`'s code (the introspection call sites): "ask the host to
run component X against read-only build-graph state Y, get back Z." What
differs is only *who implements the host side* of that WIT interface —
`wasmtime` embedded in the native binary, or the outer JS/Worker runtime
satisfying an import when `make` itself is a guest. The `make`-internal
code that calls into extensions should be written against the WIT interface
boundary only, never against `wasmtime`'s Rust API directly, so it doesn't
need to know or care which shape it's compiled for.

## Runtime choice depends on deployment shape

This is a shape-dependent answer, not a single pick:

- **Shape A (native)**: `wasmtime` is the right choice — reference
  component-model implementation, mature capability-scoped WASI filesystem
  support, and was already the "natural" choice for a native Rust binary.
- **Shape B (wasm guest)**: neither `wasmtime` nor `rusty_v8` apply as
  *embedded* runtimes, because `make` has no native process to embed
  anything in. The component host role is played by whatever hosts `make`'s
  own wasm instance — V8 via a JS component-model shim in the browser, or
  the Worker runtime. `rusty_v8` specifically requires a native host to
  embed V8 in, so it's not usable inside Shape B at all; it was a candidate
  only under the (now superseded) assumption that `make` stays a native
  process.

Net: no single runtime decision is needed today. What's needed is the WIT
interface boundary (above), designed so that Shape A can bind it to
`wasmtime` and Shape B can bind it to a host-provided import, without the
interface itself changing.

## Open questions / not yet decided

- **`dlopen`/Guile removal timing** — atomic with MVP landing, or
  deprecate-then-remove on a release boundary (see decision 3 above).
- **WIT interface concrete shape for MVP introspection** — not yet drafted;
  next step once this doc is agreed.
- **Shape B host-side implementation** — whether a `jco`-based JS shim is
  built as part of this work or treated as a separate follow-up once the
  WIT boundary exists.
- **Testing strategy across both shapes** — an extension/interface contract
  test suite that can run against both a `wasmtime`-backed host (Shape A)
  and a JS-shim-backed host (Shape B) to keep them from drifting apart.

## Out of scope

- Compiling `make` itself for wasm targets — that's #620 (tracking) and its
  sub-issues; this doc assumes that work's outcome (a wasm-guest build of
  `make` exists) but does not itself specify how that build is produced.
- Rule definition and custom recipe execution via extensions — deferred
  past the read-only-introspection MVP (decision 1 above).
