# Exercises the `-d`/`--debug` path: parsing the flag sets the global
# debug-level bitmask (now an atomic), and the build reads it to emit basic
# tracing ("Considering target file", "Must remake target", ...). Run with
# `-n` so the recipe is only printed, never executed — output stays
# deterministic and no files are written.
all: first second

first:
	@echo first

second: first
	@echo second
