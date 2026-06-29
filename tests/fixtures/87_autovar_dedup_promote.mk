# Exercises the `$^`/`$?` dedup table's ignore_mtime promotion branch in
# set_file_variables: the same prereq name (`a`) appears as BOTH a normal and
# an order-only prereq. The dedup map keeps the first (normal) `a` as the
# canonical node and, on seeing the order-only duplicate, promotes both to
# normal (clearing ignore_mtime) — so `a` lands in `$^` (deduped normals), not
# `$|` (order-only). `c` stays purely order-only. Names and order are fixed, so
# the output is byte-for-byte comparable against the C oracle.
all: a b | a c
	@echo caret=$^
	@echo plus=$+
	@echo bar=$|
a b c: ; @:
