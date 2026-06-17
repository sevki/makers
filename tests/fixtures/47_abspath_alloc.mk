# Exercises the alloc-style builtin path in expand_builtin_function
# (function.rs): $(abspath) returns a freshly malloc'ed buffer that is now
# owned via the RAII `ExpandedArg` wrapper. abspath normalizes lexically and
# does not require the paths to exist, so both binaries must agree byte-for-byte
# (they run in the same directory).
all:
	@echo abs $(abspath foo/../bar ./baz/. a/b/../c)
