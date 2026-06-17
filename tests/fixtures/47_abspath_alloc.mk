# Exercises the alloc-style builtin path in expand_builtin_function
# (function.rs): $(abspath) returns a freshly malloc'ed buffer that is now
# owned via the RAII `ExpandedArg` wrapper. The inputs are absolute paths so
# the lexical normalization is independent of the working directory (the
# differential harness runs each binary in its own temp dir), letting both
# binaries agree byte-for-byte.
all:
	@echo abs $(abspath /foo/../bar /a/b/../c /x/./y /p/q///r)
