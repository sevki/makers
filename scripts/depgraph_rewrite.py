#!/usr/bin/env python3
"""One-shot rewrite for the dependency-graph idiomatic-Rust conversion.

Converts c2rust bitfield accessor call sites on File/Dep/GoalDep/patdeps to
plain field accesses, and us_*/cs_* constants to real enum variants.
"""
import re
import sys
from pathlib import Path

SRC = Path(__file__).resolve().parent.parent / "src"

BOOLS = [
    # Dep / GoalDep / patdeps
    "changed", "ignore_mtime", "staticpattern", "need_2nd_expansion",
    "ignore_automatic_vars", "is_explicit", "wait_here",
    # File
    "builtin", "precious", "loaded", "unloaded", "low_resolution_time",
    "tried_implicit", "updating", "updated", "is_target", "cmd_target",
    "phony", "intermediate", "secondary", "notintermediate", "dontcare",
    "ignore_vpath", "pat_searched", "no_diag", "was_shuffled", "snapped",
    "suffix",
]

ENUM_CONSTS = {
    "us_success": "UpdateStatus::Success",
    "us_none": "UpdateStatus::None",
    "us_question": "UpdateStatus::Question",
    "us_failed": "UpdateStatus::Failed",
    "cs_not_started": "CommandState::NotStarted",
    "cs_deps_running": "CommandState::DepsRunning",
    "cs_running": "CommandState::Running",
    "cs_finished": "CommandState::Finished",
}

CAST_TYPES = r"(?:update_status_0|update_status|cmd_state)"
INT = r"(?:::core::ffi::c_int|::core::ffi::c_uint|libc::c_int|libc::c_uint|i32|u32)"
# 0/1 literal with arbitrary chained `as <int>` casts
ONE = rf"1(?:\s+as\s+{INT})*"
ZERO = rf"0(?:\s+as\s+{INT})*"

def rewrite(text: str) -> str:
    # --- enum constants -----------------------------------------------------
    # strip casts of enum consts to their old integer alias types
    text = re.sub(
        rf"\b(us_success|us_none|us_question|us_failed|cs_not_started|cs_deps_running|cs_running|cs_finished)\s+as\s+{CAST_TYPES}\b",
        r"\1", text)
    for old, new in ENUM_CONSTS.items():
        text = re.sub(rf"\b{old}\b", new, text)

    # --- enum field setters -------------------------------------------------
    text = re.sub(r"\.set_(update_status|command_state)\(([^()]*(?:\([^()]*\)[^()]*)*)\)",
                  r".\1 = \2", text)
    # --- enum field getters -------------------------------------------------
    text = re.sub(r"\.(update_status|command_state)\(\)", r".\1", text)

    # --- flags (plain c_uint field) ----------------------------------------
    text = re.sub(r"\.set_flags\(([^()]*(?:\([^()]*\)[^()]*)*)\)", r".flags = \1", text)
    text = re.sub(r"\.flags\(\)", r".flags", text)

    names = "|".join(BOOLS)

    # --- bool setters ---------------------------------------------------
    text = re.sub(rf"\.set_({names})\(\s*{ONE}\s*\)", r".\1 = true", text)
    text = re.sub(rf"\.set_({names})\(\s*{ZERO}\s*\)", r".\1 = false", text)
    # generic fallback: .set_x(expr) -> .x = (expr) != 0
    text = re.sub(rf"\.set_({names})\(([^()]*(?:\([^()]*\)[^()]*)*)\)",
                  r".\1 = (\2) != 0", text)

    # --- bool getters in comparison contexts (before plain getter pass) -----
    # `(*r).x() as c_int != 0` / `r.x() != 0` -> `(*r).x` / `r.x`
    text = re.sub(rf"\.({names})\(\)(?:\s+as\s+{INT})?\s*!=\s*0\b", r".\1", text)
    # `(*r).x() == 0` -> `!(*r).x`
    text = re.sub(rf"(\(\*[A-Za-z_0-9]+\))\.({names})\(\)(?:\s+as\s+{INT})?\s*==\s*0\b",
                  r"!\1.\2", text)
    text = re.sub(rf"\b([A-Za-z_][A-Za-z_0-9]*)\.({names})\(\)(?:\s+as\s+{INT})?\s*==\s*0\b",
                  r"!\1.\2", text)
    # plain getter
    text = re.sub(rf"\.({names})\(\)", r".\1", text)

    # `(*r).x as c_int != 0` -> `(*r).x`  (bool as int round-trips)
    text = re.sub(rf"\.({names})\s+as\s+{INT}\s*!=\s*0\b", r".\1", text)

    # --- type tokens ---------------------------------------------------
    text = re.sub(r"(?<!\.)\bcmd_state\b", "CommandState", text)
    text = re.sub(r"(?<!\.)\bupdate_status_0\b", "UpdateStatus", text)
    text = re.sub(r"(?<!\.)\bupdate_status\b", "UpdateStatus", text)

    # CamelCase the dep-graph type names, but only in type positions —
    # `file`, `dep`, `rule` etc. are also common variable names.
    renames = {
        "file": "File",
        "dep": "Dep",
        "commands": "Commands",
        "rule": "Rule",
        "goaldep": "GoalDep",
        "nameseq": "NameSeq",
        "patdeps": "PatDeps",
        "tryrule": "TryRule",
    }
    for tok, new in renames.items():
        text = re.sub(rf"\*mut {tok}\b", f"*mut {new}", text)
        text = re.sub(rf"\*const {tok}\b", f"*const {new}", text)
        text = re.sub(rf"::<{tok}>", f"::<{new}>", text)
        text = re.sub(rf":\s*{tok}\s*=\s*{tok}\s*\{{", f": {new} = {new} {{", text)
        text = re.sub(rf"=\s*{tok}\s*\{{\n", f"= {new} {{\n", text)
    return text


def main():
    files = sys.argv[1:] or sorted(SRC.glob("*.rs"))
    for f in files:
        p = Path(f)
        orig = p.read_text()
        new = rewrite(orig)
        if new != orig:
            p.write_text(new)
            print(f"rewrote {p.name}")


if __name__ == "__main__":
    main()
