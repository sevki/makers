#!/usr/bin/env python3
"""Pre-pass: drop duplicate goaldep structs and us_*/cs_* const blocks."""
import re
from pathlib import Path

SRC = Path(__file__).resolve().parent.parent / "src"

GOALDEP_RE = re.compile(
    r"#\[derive\(Copy, Clone, BitfieldStruct\)\]\n#\[repr\(C\)\]\npub struct goaldep \{.*?\n\}\n",
    re.S,
)

CONST_LINES = [
    re.compile(r"^pub type cmd_state = ::core::ffi::c_uint;\n", re.M),
    re.compile(r"^pub const cs_\w+: cmd_state = \d;\n", re.M),
    re.compile(r"^pub type update_status = ::core::ffi::c_uint;\n", re.M),
    re.compile(r"^pub type update_status_0 = u32;\n", re.M),
    re.compile(r"^pub const us_\w+: (?:update_status_0|update_status) = \d;\n", re.M),
]

ALIAS_LINES = [
    re.compile(r"^pub type file = File;\n", re.M),
    re.compile(r"^pub type dep = Dep;\n", re.M),
    re.compile(r"^pub type commands = Commands;\n", re.M),
]

# implicit.rs has a byte-identical clone of rule.rs's `rule` struct
RULE_CLONE_RE = re.compile(
    r"#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct rule \{\n"
    r"    pub next: \*mut rule,\n.*?\n\}\n",
    re.S,
)

NAMESEQ_IMPORT_RE = re.compile(r"^pub use crate::file::nameseq;\n", re.M)

for p in sorted(SRC.glob("*.rs")):
    if p.name == "file.rs":
        continue
    text = orig = p.read_text()
    text, n = GOALDEP_RE.subn("pub use crate::file::GoalDep;\n", text)
    had_consts = False
    for rx in CONST_LINES:
        text, k = rx.subn("", text)
        had_consts = had_consts or k > 0
    for rx in ALIAS_LINES:
        text = rx.sub("", text)
    if p.name == "implicit.rs":
        text = RULE_CLONE_RE.sub("pub use crate::rule::Rule;\n", text)
    text = NAMESEQ_IMPORT_RE.sub("pub use crate::file::NameSeq;\n", text)
    if had_consts:
        # one import where the alias block used to live; top of file is fine
        text = "pub use crate::file::{CommandState, UpdateStatus};\n" + text
    if text != orig:
        p.write_text(text)
        print(f"prepass: {p.name} (goaldep={'y' if n else 'n'}, consts={'y' if had_consts else 'n'})")
