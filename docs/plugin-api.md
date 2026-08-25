# The `makers` build plugin interface

Status: proposed, with the analysis phase implemented.
Interface: [`wit/makers-plugin/`](../wit/makers-plugin) — `makers:plugin@1.0.0`.
Host: [`src/plugin.rs`](../src/plugin.rs), [`src/plugin/host.rs`](../src/plugin/host.rs).
SDK: [`makers-plugin/`](../makers-plugin). Examples: [`plugins/`](../plugins).
Supersedes the `makers:introspection@0.1.0` draft in
[#647](https://github.com/sevki/makers/pull/647); builds on the architecture
in [`docs/wasm-extension-system.md`](wasm-extension-system.md).

---

## 1. What this is

A plugin system for `make` built on WebAssembly components: plugins are
`.wasm` components with a WIT-typed interface, written in any language with
a component toolchain, loaded and mediated by `make` itself.

The comparison everyone reaches for is Bazel and Starlark, and it is the
right one — Starlark is the only widely deployed answer to "let people
extend a build system without letting them break it". This document takes
Bazel's good ideas on purpose (aspects, providers, depsets, declared
actions, phases) and argues that the component model gets the *hard* part —
the boundary between the build tool and third-party code — meaningfully
better than an embedded language can.

The argument is not "wasm is modern". It is specific, and it is section 4.

---

## 2. What the first cut got wrong

[#647](https://github.com/sevki/makers/pull/647) landed a working host, a
working SDK, and a working plugin, which is why it is a good base: the wasm
plumbing is right. The *interface* it plumbed is not, and every problem
below showed up in the one plugin that was written against it.

**It copies the graph into the guest.** `visit-file` took a `file` record
carrying `name`, `stem`, and both dependency lists **by value**, on every
node. A plugin that reads only `file.name` still pays for every string in
every dependency list. On LLVM (~40k translation units, ~150 transitive
headers each) that is millions of strings allocated in guest linear memory
and immediately dropped. Bazel hit this exact wall and answered it with
`depset`, whose entire reason for existing is that concatenating transitive
dependency lists is quadratic in graph size.

**It has no output.** The example plugin writes its Graphviz document to
**stderr**, and the test asserts on stderr. That is not a rough edge, it is
a missing concept: plugin output cannot be redirected, cannot be written
atomically, interleaves with make's diagnostics under `-j`, and cannot be
discovered or cached by the host. Every plugin anyone actually wants —
compile databases, dependency graphs, profiles, coverage manifests — exists
to produce a file.

**Its traversal order does not match its documentation.** The host walks
with an explicit `Vec` stack and `pop()`, announcing a node's own edges in
makefile order but then processing sibling *subtrees* in reverse. On the
repository's own fixture, `outdir`'s subtree is walked before `main.o`'s,
though `main.o` is written first. The WIT comment promises depth-first
order and claims that only the host's real traversal order can answer
`$<`/`$^`/`.WAIT` order meaningfully. Both cannot be true at once.

**It cannot accumulate.** Hooks return `result<_, string>`. A plugin cannot
publish anything for another node — or another plugin — to read, so every
non-trivial analysis (transitive include paths, link lines, licence sets)
has to re-derive everything from scratch in guest memory, and two plugins
can never cooperate. This is what Bazel providers are for.

**It is one anonymous plugin, configured by an environment variable.**
`MAKERS_WASM_EXTENSION=path`: no name, no settings, no way to run two, no
way to say what a plugin may do.

**It grants ambient authority anyway.** The host links the full WASI p2
surface and inherits stderr. Nothing states, checks, or records what a
plugin can reach. This is the one thing wasm buys over `dlopen` and it was
left on the table.

**Its failure policy is hardcoded.** Errors are caught, printed, and the
build continues. That is correct for a profiler and makes every lint,
policy, or licence-gate plugin unimplementable.

None of this is a criticism of shipping it: it was explicitly an MVP, and
the way you find these is by writing a plugin. This document is what the
plugin found.

---

## 3. The design

Ten decisions. Each one names the real-world plugin that forces it.

### 3.1 Handles, not payloads

`node` and `node-set` are WIT **resources**. A node is a 4-byte index; every
field is a method the guest calls only if it wants the value.

> *Use case: `compile_commands.json` over LLVM.* The plugin needs
> `node.recipe()` and one prerequisite name per compile step. Under the
> record design it receives every transitive dependency name of every node.
> Under the handle design it makes roughly two calls per node it cares about
> and none for the rest.

`node-set` is a real nested set: `union` is O(1) and links two sets rather
than copying their members; `to-list` is the only operation that flattens,
and it memoises. `transitive-deps` is lazier still — it names a closure that
has not been walked, so a plugin that unions ten closures and then filters
pays for one traversal instead of ten.

> *Use case: transitive include paths.* Every node's answer is "my own
> `-I` flags plus my dependencies' answers". With copying lists that is
> O(nodes × depth). With `union` it is O(edges), which is the difference
> between usable and not on a large tree. This is exactly Bazel's `depset`
> argument, and Bazel's documentation warning against flattening a depset
> inside a loop is the same warning that applies to `to-list` here.

Flattening order is part of the contract — left operand first, first
occurrence wins — because plugins turn these lists into **files**. A link
line, a compile database and a generated Ninja file are all order-sensitive,
and an unspecified order makes every one of them differ run to run for no
reason.

### 3.2 Identity is content-addressed and stable; handles are not

`node.id()` is the host's own BLAKE3-derived `FileId`, hex-encoded: stable
across runs and across host restarts, safe to persist in an artifact or use
as a cache key. Handles are valid only for the hook they were passed to.

> *Use case: an incremental IDE index.* A plugin that writes an index keyed
> by node id can diff its previous output against the current graph without
> re-deriving names, and can survive the host renaming a target through
> VPATH resolution.

### 3.3 Phases

`plugin-info.phases` names which of make's phases a plugin participates in:
`read` (makefiles being evaluated), `analyze` (graph resolved), `execute`
(targets being updated). This mirrors Bazel's loading/analysis/execution
split, and for the same reason: the three have genuinely different APIs, and
pretending otherwise is how you end up with one hook that is wrong for
everybody.

Only `analyze` is hosted today. Declaring the other two now costs nothing
and means adding them later is not a breaking change — a plugin that says
`[analyze]` keeps meaning exactly what it means now.

### 3.4 Aspects: dependency order and publication

`analyze(node)` is called once per node reachable from the goals, in
**dependency order**: every prerequisite of a node is analysed before the
node, and among siblings the makefile's own edge order (which is `$^`
order, and whose first element is `$<`) is preserved.

That ordering is the whole point. It is what makes `dep.providers()`
meaningful, and therefore what makes bottom-up accumulation expressible at
all. Bazel's aspects work this way; the pre-order walk in #647 cannot.

Hooks **publish** rather than mutate: `analyze` returns providers, and no
hook can skip a node, reorder the walk, or add an edge. Two plugins can
therefore run in one pass and compose without either being able to change
what the other sees about the build. It is also what makes the phase
cacheable — see 3.9.

A dependency cycle is reported and dropped rather than propagated, matching
make's own `Circular X <- Y dependency dropped`. A plugin pass that refused
to run on graphs make itself builds happily would be useless on exactly the
makefiles most worth inspecting.

### 3.5 Providers

A provider is `(id: string, payload: list<u8>)` attached to a node. Ids are
namespaced (`makers:cc/compile-command`) and the host rejects unnamespaced
ones, so two unrelated plugins cannot collide on `"info"`.

Providers published by one plugin are visible to the next, in configuration
order, on the same graph. **This is the composition story**, and it is
demonstrated by the two example plugins: `compile-commands` publishes
`makers:cc/compile-command` on every compile step; `graphviz-export` draws
any node carrying that provider as a compile step. `graphviz-export` does
not depend on `compile-commands`, does not decode its payload, and works
perfectly well without it.

> *Use case: a licence auditor and a SBOM writer.* One plugin knows how to
> map source paths to licences; another knows the CycloneDX schema. Neither
> should have to contain the other. In Bazel they would share a
> `provider()`; here they share a provider id.

**Why opaque bytes and not a WIT variant.** The value of Bazel providers is
that `CcInfo`/`JavaInfo`/your own are third-party extensible: the ruleset
author declares the type and the build system never looks inside. Encoding
provider payloads in WIT would put every new provider on `makers`' release
schedule. Bytes keep that on the plugin author's schedule and still let the
host do the one thing it needs to do without understanding a payload: hash
it. The cost — producer and consumer must agree on an encoding out of band —
is the same cost Bazel providers have (both sides must load the same `.bzl`),
and the namespaced `id` is where that contract is versioned.

### 3.6 Capabilities, granted by the host

A plugin **declares** what it needs in `describe()`. The host instantiates
the component in a store with *no* authority at all to make that call,
validates the answer, grants the intersection of what was asked for and what
the operator allowed, reports the difference, and only then builds the real
store.

The second instantiation is not waste — compilation is cached in the
`Component`, so it costs one instantiation — and it buys the property that
no capability-bearing store exists before the host has seen the request. A
manifest baked into a custom section at build time would be cheaper and
would describe what the *source* said rather than what the shipped artefact
asks for.

Capabilities are few on purpose; a split nobody exercises is theatre. Each
of these separates plugins that exist:

| capability | gates | a plugin that needs it |
|---|---|---|
| `read-recipes` | `node.recipe` | a compile database |
| `read-variables` | `vars.get`, `node.variable` | a toolchain reporter |
| `expand-variables` | `vars.expand` | a Ninja/BUILD generator |
| `read-environment` | `session.env` | a CI-aware reporter |
| `read-file-content` | a read-only WASI preopen of the working directory | a header or licence scanner |
| `wall-clock` | the WASI clock itself | a profiler |
| `write-outputs` | `artifacts.open` | anything producing a file |
| `fail-build` | letting an error set the exit status | a policy gate |

Two of these are worth dwelling on.

`expand-variables` is the only one the host does not grant even to a plugin
that asks politely, because `expand("$(shell curl … | sh)")` is arbitrary
code execution *outside* the wasm sandbox, in make's own process. A sandbox
is worth nothing if the guest can ask the host to run a shell for it. It
exists at all because a plugin that regenerates a build description has to
turn `$(CC) $(CFLAGS) -c $<` into a command line, and reimplementing make's
expander inside every such plugin is both a correctness disaster and a
maintenance one.

`wall-clock` is enforced at the WASI clock, not at a `makers:plugin`
function. A capability checked only at the interface that names it is not a
capability: `cargo component` gives every plugin a `wasi:clocks` import
whether its code mentions time or not, so an ungranted plugin gets a clock
stopped at the epoch. Stopped rather than absent, because a missing clock
makes Rust's `std` panic on paths a plugin never asked for.

The default grant is `read-recipes`, `read-variables`, `write-outputs` — the
things a plugin could learn from `make -p` anyway, plus its declared output.
Everything reaching outside the makefiles make already read is opt-in.

### 3.7 Declared outputs

A plugin asks for `"database"`; the host decides that means
`./compile_commands.json`, or whatever `--plugin-arg` said, buffers the
writes, and publishes via a temporary file and an atomic rename.

> *Use case: `compile_commands.json` again.* A half-written database makes
> clangd report thousands of phantom errors across the tree. A stream
> abandoned because the plugin trapped, ran out of fuel, or returned an
> error publishes nothing, and the previous database stays valid.

Declaring outputs rather than granting filesystem write access buys three
things: the plugin needs no write capability at all, the artifact set is
knowable *before* the plugin runs (so make can report or clean it), and the
host can in principle serve a cache hit without instantiating the component.

### 3.8 Diagnostics and a declared failure policy

`diagnostics.emit` takes a severity, a message and an optional makefile
`location` — make already tracks a `Floc` for every recipe, goal and
per-target variable, so `Makefile:42: recipe uses an absolute path` costs
nothing to produce and is a clickable diagnostic in every editor.

Reporting is separate from failing. A lint plugin needs to report many
findings and still finish; a hook that could only signal a problem by
returning would stop at the first one.

Whether an error *matters* is declared, not fixed:

* `advisory` (the default) — reported, exit status unaffected. Correct for a
  profiler that cannot open its trace file.
* `fatal` — sets make's exit status, and requires the `fail-build`
  capability. Correct for a licence gate that cannot read its allowlist.

A manifest declaring `fatal` without requesting `fail-build` is rejected at
load rather than silently downgraded, because a policy gate that silently
stops gating is worse than one that refuses to start.

### 3.9 Determinism the host can check

`session.input-digest()` is BLAKE3 over everything the analysis phase can
observe about the graph — node identity, dependency structure, edge flags,
recipe text, per-target variables — plus this instance's settings. A plugin
may declare `deterministic: true`, promising its declared outputs are a
function of what it read through these interfaces; the host may then skip it
entirely when the digest matches the one recorded beside its previous
outputs.

`makers` is unusually well placed to do this: the graph is already
content-addressed (`FileId`, `DepId`, `RuleId` are BLAKE3 hashes) and
analyses already run through salsa, so per-node memoisation of plugin
results is a natural extension rather than new machinery.

The promise is partially **checked**, not merely trusted: `deterministic`
together with `wall-clock` or `read-environment` is rejected at load,
because both are ways to make output depend on something the digest does not
cover.

> *Use case: a no-op rebuild.* `make` with a compile-database plugin on a
> 10k-target tree should cost nothing when nothing changed. Bazel gets this
> from Skyframe memoising a pure Starlark evaluation. Here it comes from the
> host being able to *name* the plugin's inputs — which is a weaker
> requirement than purity, and admits plugins Starlark cannot express.

### 3.10 Metering and versioning

Each instance runs with a fuel budget and a linear-memory ceiling, both
configurable per instance. Fuel is deterministic — a runaway plugin fails
the same way on a laptop and on a CI runner — which a wall-clock timeout is
not.

> *Use case: a plugin from a third-party repository.* "Someone's aspect made
> analysis take four minutes" is a routine Bazel complaint with no per-rule
> lever to pull; Starlark evaluation has no per-extension budget. Here the
> operator sets one.

Interface versioning is the component model's job, not a version string two
parties agree to keep honest: a plugin's imports name
`makers:plugin/graph@1.0.0`, so a host that does not provide it fails at
instantiation with an exact message. `plugin-info` therefore carries the
*plugin's* version and not the interface's.

---

## 4. Why this is better than Starlark

Starlark is a good design and this is not a claim that embedding a language
was a mistake. It is a claim that the boundary is in a better place.

**Hermeticity by mediation instead of by amputation.** Starlark is
deterministic because it *cannot* do I/O, read the clock, or see the
environment. That works, and it is why `repository_rule` and module
extensions exist as a separate, privileged, famously hand-invalidated escape
hatch: the moment a rule genuinely needs the outside world, Starlark has
nothing to offer it. A component can be granted a scoped, read-only handle
to make's working directory — and *because the grant goes through the
host*, the host can record it. That is strictly more expressive than "no
I/O" and strictly more analysable than "unrestricted `ctx.execute`".

> *Use case: header scanning.* Deciding that `main.o` depends on `config.h`
> means reading `main.c`. In Bazel this cannot be a Starlark rule at all —
> include scanning is C++ code inside Bazel. Here it is a plugin with
> `read-file-content`, and it is still a plugin.

**Any language, shipped as a binary.** A `.bzl` file is source, evaluated by
Bazel's own interpreter, in one language. A component is a compiled
artefact: Rust, C, Go via TinyGo, anything with a component toolchain. It
has a content hash, so "which plugin ran" is a fact rather than a directory
state. Where a Bazel rule needing a real algorithm shells out to a tool it
must first build, a plugin can just contain the algorithm.

**A real sandbox, not a restricted language.** Starlark's isolation comes
from what the language omits, so it is only as good as the standard
library's exposed surface. A component's isolation is the wasm sandbox: a
plugin cannot read memory it was not handed, cannot open a file it was not
given, and cannot outrun its fuel budget. This is also the direct answer to
what `make` has today — `dlopen`'d `gmk_*` plugins get the host's entire
address space the moment `dlopen` returns.

**Metering.** Section 3.10.

**Typed, versioned, negotiated interfaces.** A `.bzl` calling a field that
no longer exists fails during analysis with an `AttributeError`. A component
importing an interface the host does not provide fails at instantiation,
before it runs, naming the interface and version.

**Where Starlark still wins, honestly.** Editing a `.bzl` and re-running is
faster than a compile-to-wasm cycle. Starlark's data model is friendlier
than opaque provider payloads. And Bazel's ecosystem is enormous — this
interface has two plugins. The bet is that a build tool's extension boundary
is worth getting right even at some cost in iteration speed, because the
things people extend build systems *with* (compile databases, dependency
analyses, policy gates, caches) are programs, not scripts.

---

## 5. The two example plugins

**`plugins/compile-commands`** produces `compile_commands.json`. This is the
plugin that justifies the interface: a JSON Compilation Database is what
clangd, clang-tidy, IWYU and every C/C++ IDE integration read, and `make`
cannot produce one. The workarounds in the wild are all bad in the same way
— `bear` intercepts every `exec` the build performs and reconstructs intent
from it, `compiledb` re-parses `make -n` output — and both require a full or
fully dry build before they know anything. All of them are guessing at a
graph make already holds in memory.

It uses `node.recipe()` (capability-gated), `node.dep-edges()` to tell the
source apart from an order-only `| build/` prerequisite that must *not*
appear as an input, `node.variable()` so `debug.o: CFLAGS += -O0` comes out
right for that target, `session.working-directory()` because the schema
requires it, and one declared, atomically published output.

**`plugins/graphviz-export`** is the #647 plugin rebuilt. It writes a
declared artifact instead of stderr, draws order-only prerequisites as the
ordering constraints they are (the old `dep` record had no flags), flags
prerequisites that are neither targets nor existing files, and annotates any
node carrying `makers:cc/compile-command` — without knowing what produces
it.

---

## 6. Configuration

Environment variables are the surface for this slice. The command-line and
makefile surfaces are specified below and land separately: adding options to
make's own getopt table has its own blast radius and does not belong in the
same change as the interface.

```sh
MAKERS_PLUGINS="compdb=./compile_commands.wasm,graph=./graphviz.wasm"
MAKERS_PLUGIN_ARGS="compdb.out.database=build/compile_commands.json;graph.rankdir=TB"
MAKERS_PLUGIN_ALLOW="compdb:read-recipes,read-file-content"
MAKERS_PLUGIN_DENY="graph:write-outputs"
MAKERS_PLUGIN_VERBOSE=1
MAKERS_WASM_EXTENSION=./plugin.wasm   # the #647 debug tap, as instance `default`
```

Plugins run in configuration order, sharing one provider map. Settings are
per instance, so one compiled component can be loaded twice with different
configuration — the separation Bazel draws between a `.bzl` rule and the
attributes a `BUILD` file passes it, and what lets a plugin be reused rather
than forked per project.

Planned:

```
--plugin NAME=PATH
--plugin-arg NAME:KEY=VALUE
--plugin-allow NAME:CAP[,CAP…]
--plugin-list
```

and, in a makefile:

```make
.PLUGIN: compdb ./tools/compile_commands.wasm
.PLUGIN_ARGS: compdb out.database=build/compile_commands.json
```

---

## 7. Staged: the other two phases

These are designed, not implemented. They are sketched here so that the
interface that exists is checkably a subset of the interface that is coming,
rather than something that will have to be broken to grow.

### 7.1 The `reader` world — extending the language

```wit
interface functions {
    /// Register a makefile function. The host dispatches `$(name args…)`
    /// to the plugin during expansion.
    record function-decl { name: string, min-args: u32, max-args: u32, expand-args: bool }
    call: func(name: string, args: list<string>) -> result<string, error>;
}
```

This is the replacement for `gmk_add_function`, the most-used part of the
`dlopen` API this system supersedes, and dropping it would be a regression
against the thing being replaced. The host already has the machinery
(`function::define_new_function`, and `gmk_func_ptr` passes the function
name, so one trampoline can dispatch by name).

> *Use case: `$(sha256 file)` and `$(git-rev)`.* Both are `$(shell …)`
> today, which means they are uncacheable and re-run on every expansion. As
> plugin functions with declared inputs they are memoisable.

The interesting design question is not the mechanism but the invalidation:
a plugin function's result must be part of the input digest, which is the
same problem `repository_rule` invalidation is, and worth solving properly
rather than quickly.

### 7.2 The `executor` world — actions and strategies

```wit
record action {
    target: string,
    inputs: list<string>,
    outputs: list<string>,
    command: list<string>,
    env: list<tuple<string, string>>,
}
/// Return `none` to decline and let make run it normally.
try-execute: func(a: action) -> option<result<action-result, error>>;
observe: func(a: action, r: action-result);
```

Bazel calls these spawn strategies, and the list of things they enable is
the list of things people wish `make` had:

> *`ccache`/`sccache`.* A content-addressed compile cache is exactly
> "compute the action key, serve it if present". `make` cannot express this
> because its staleness model is mtimes; a plugin with the action's inputs
> and command can express it, and `makers`' content-addressed ids and
> BLAKE3 hashing are already most of the way there.
>
> *Remote execution.* Same hook, different backend.
>
> *A build profile.* `observe` plus timestamps is a Chrome trace — the
> thing `bazel --profile` produces and `make` has no equivalent of.
>
> *Persistent workers.* Bazel's biggest single compile-throughput win. The
> plugin holds the process; the host hands it actions.

`observe` is deliberately separable from `try-execute`: observation needs no
authority to change what runs, so a profiler should not have to ask for the
capability that would let it substitute compilers.

---

## 8. Migration

* **From `makers:introspection@0.1.0` (#647).** `visit-file` becomes
  `analyze`, minus the eager record; `visiting-child` becomes
  `node.dep-edges()` read from inside `analyze`; `visit-done` becomes
  `finish`. `MAKERS_WASM_EXTENSION` keeps working as the instance `default`.
  The old world is removed rather than kept alongside: it has one plugin, in
  this repository, and it is ported here.
* **From the `dlopen` `gmk_*` API** (`src/load.rs`, `src/loadapi.rs`) and
  **Guile** (`src/guile.rs`, already a disabled stub). Both are to be
  replaced, per the decision recorded on
  [#633](https://github.com/sevki/makers/issues/633). `gmk_add_function` is
  the one part with no replacement yet — see 7.1 — so retiring the `dlopen`
  path should wait for the `reader` world rather than land with this change.

---

## 9. Open questions

* **Cross-run caching.** `input-digest` exists and `deterministic` is
  declared and validated, but the host does not yet keep a digest sidecar
  beside published outputs, so nothing is skipped yet. The mechanism is
  small; the interaction with `--always-make`, `-B` and remade makefiles is
  the part worth thinking about.
* **Provider payload conventions.** Namespacing is enforced; encoding is not
  suggested. A recommended encoding (and an SDK helper for it) would make
  cross-plugin composition much likelier to actually happen.
* **Enumerating global variables.** `vars.get` is by name. A plugin
  exporting a `.env` or a `BUILD` file wants the whole set, which needs an
  iteration path over the C hash table that does not exist yet.
* **Non-UTF-8 target names.** They arrive lossily converted, as make's own
  display path converts them; `node.id()` stays byte-exact. Byte-exact
  *names* would need `list<u8>` accessors, which is worth doing only if a
  real plugin needs it.
* **Shape B.** `world plugin-host` names the host side so that the
  `jco`-based shim from `docs/wasm-extension-system.md` has something
  concrete to implement. Whether it can is unproven.
