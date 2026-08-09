# Opt-in divergences from the C oracle

The port's contract is byte-identical output against the in-tree C make
(`fixtures-diff` is the hard gate). Occasionally the oracle is the side that is
wrong: the C code has a bug, the port would naturally do the correct thing, and
"match the oracle" and "be correct" stop being the same instruction.

The rule for those cases:

- **The default matches the oracle, bug included.** Fidelity is the product;
  nothing silently behaves differently from the make being replaced, and the
  fixture stays in the differential run instead of being quarantined.
- **The correct behaviour ships behind an opt-in environment variable**, off
  unless explicitly set, so the divergence is reachable, tested, and documented
  rather than lost in an issue thread.
- **Both sides are pinned by tests** — the default by the differential fixture,
  the opt-in by a `tests/rs_integration.rs` case that sets the variable.

Each variable is off when unset, empty, or `0`; any other value turns it on.

## `MAKERS_AR_GLOB_MEMBER_NAMES` — archive-member wildcard names (#460)

`$(wildcard libdiff.a(*.o))` over an archive with five members expands to the
archive name five times:

```
$ make -f - <<<'all: ; @echo "[$(wildcard libdiff.a(*.o))]"'
[libdiff.a libdiff.a libdiff.a libdiff.a libdiff.a]
```

`ar_glob_match` (`src/ar.c`) builds each element as
`concat (4, arname, "(", mem, ")")`, so `archive(member)` is what the chain is
supposed to carry. The names are then discarded in `src/read.c`, in the loop
that attaches the chain:

```c
while (1)
  {
    if (! cachep)
      found->name = xstrdup (concat (2, prefix, name));
    else if (prefix)
      found->name = strcache_add (concat (2, prefix, name));
    ...
  }
```

`name` was reassigned to the archive name earlier in the same block
(`ar_parse_name (name, &arname, &memname); name = arname;`), so every element's
correct name is overwritten. The intended operand is almost certainly
`found->name` — that is the only reading under which the loop's own comment
("Massage names if necessary") and the `prefix` handling make sense.
`$(wildcard)` reaches this via `func_wildcard` → `string_glob` →
`PARSE_FILE_SEQ(..., PARSEFS_NOSTRIP|PARSEFS_NOCACHE|PARSEFS_EXISTS)`, and
`PARSEFS_NOCACHE` means `cachep` is false, so the first branch always fires.
The one case that escapes the rewrite is `cachep` with no `prefix`, where
`found->name` is left alone.

Setting the variable keeps the member names:

```
$ MAKERS_AR_GLOB_MEMBER_NAMES=1 make -f - <<<'all: ; @echo "[$(wildcard libdiff.a(*.o))]"'
[libdiff.a(Mid.o) libdiff.a(alpha.o) libdiff.a(beta.o) libdiff.a(mid.o) libdiff.a(zeta.o)]
```

Ordering is unaffected either way: elements are sorted by make's
`alpha_compare` ordering (`misc::alpha_cmp`, a plain byte comparison, so `M`
sorts ahead of every lowercase initial).

- Implementation: `crate::ar::ar_glob_member_names` (`src/ar.rs`), applied in
  `parse_file_seq` (`src/read.rs`).
- Tests: `ar_glob_member_sort_matches_oracle` (default) and
  `ar_glob_member_names_opt_in` (opt-in) in `tests/rs_integration.rs`; the
  `ar-glob-member-sort` fixture diffs the default against the C oracle.
