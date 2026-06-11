#!/usr/bin/env python3
"""Remove crate-internal extern "C" declarations whose signatures mention the
dependency-graph types (File/Dep/GoalDep/Rule/Commands), replacing them with
direct imports of the defining module's items."""
import re
from pathlib import Path

SRC = Path(__file__).resolve().parent.parent / "src"
MOD_NAME = {"main": "make_main"}
GRAPH_TYPES = re.compile(r"\*(?:mut|const) (?:File|Dep|GoalDep|Rule|Commands)\b")

# 1. index #[no_mangle] definitions: name -> module
defs = {}
def_re = re.compile(
    r'#\[no_mangle\]\s*\npub (?:unsafe )?(?:extern "C" )?(?:fn|static mut|static) (\w+)'
)
for p in sorted(SRC.glob("*.rs")):
    mod = MOD_NAME.get(p.stem, p.stem)
    for m in def_re.finditer(p.read_text()):
        defs.setdefault(m.group(1), mod)

# 2. per file, remove extern decls of indexed symbols whose sig mentions graph types
fn_decl_re = re.compile(r"\n(    fn (\w+)\((?:[^()]|\([^()]*\))*?\)(?: -> [^;]+?)?;)", re.S)
static_decl_re = re.compile(r"\n(    static mut (\w+): [^;]+?;)", re.S)

for p in sorted(SRC.glob("*.rs")):
    mod = MOD_NAME.get(p.stem, p.stem)
    text = p.read_text()
    imports = []

    def maybe_remove(m):
        decl, name = m.group(1), m.group(2)
        target = defs.get(name)
        if target is None or target == mod:
            return m.group(0)
        if not GRAPH_TYPES.search(decl):
            return m.group(0)
        imports.append((target, name))
        return "\n"

    new = fn_decl_re.sub(maybe_remove, text)
    new = static_decl_re.sub(maybe_remove, new)
    if imports:
        lines = "".join(
            f"pub use crate::{t}::{n};\n" for t, n in sorted(set(imports))
        )
        new = lines + new
        p.write_text(new)
        print(f"{p.name}: removed {len(imports)} decls")
