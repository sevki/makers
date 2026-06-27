# Exercises the automatic-variable lists that `autovar_dep_name` feeds —
# `$^` (normal prereqs, deduped), `$+` (normal prereqs, duplicates kept), and
# `$|` (order-only prereqs). The helper now returns a safe `&[u8]` slice of each
# dependency name instead of the c2rust `(*const c_char, len)`; this drives it
# once per prereq across all three lists. Names and order are fixed, so the
# output is byte-for-byte comparable against the C oracle.
all: a a b | c
	@echo caret=$^
	@echo plus=$+
	@echo bar=$|
a b c: ; @:
