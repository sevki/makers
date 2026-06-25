# Exercises the `-l`/`--load-average` option, whose unset default and
# no-argument value are the read-only `default_load_average` (now an immutable
# `static`, formerly a `static mut`). For a non-parallel build the load limit
# has no observable effect, so a plain build (which reads the option table's
# `default_value`) and a no-argument `-l` build (which reads `noarg_value`) both
# match the C oracle byte-for-byte; the point is that the option-table entry
# referencing `default_load_average` resolves identically.
all:
	@echo built $@
