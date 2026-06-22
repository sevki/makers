# Exercises $(notdir ...) and $(suffix ...), both of which route through the
# single func_notdir_suffix handler now selected via the typed NotdirSuffix AST
# classifier (replacing the raw `*funcname == 's'` first-byte dispatch).

LIST := src/a.c include/b.h plain Makefile dir/sub/c.tar.gz noext.

# $(notdir) keeps the tail after the last directory separator.
ND := $(notdir $(LIST))
# $(suffix) keeps each token's suffix from its last '.', dropping tokens
# with no '.'. A trailing-dot token yields an empty suffix.
SF := $(suffix $(LIST))

all:
	@echo notdir=[$(ND)]
	@echo suffix=[$(SF)]
