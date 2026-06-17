# Exercises find_percent_cached's backslash-collapse path: an escaped `\%` in a
# target name is a *literal* percent, not a pattern stem. Both binaries must
# unescape `foo\%bar` to the ordinary target `foo%bar` (no pattern rule) and
# build it as the prerequisite of `all`.
all: foo%bar

foo\%bar:
	@echo built $@
