# Exercises $(basename ...) and $(dir ...), both of which route through the
# single func_basename_dir handler now selected via the typed BasenameDir AST
# classifier (replacing the raw `*funcname == 'b'` first-byte dispatch).

LIST := src/a.c include/b.h plain Makefile dir/sub/c.tar.gz noext. ./rel.txt

# $(dir) keeps the directory part up to and including the last separator,
# defaulting to ./ when a token has no separator.
DR := $(dir $(LIST))
# $(basename) strips the extension from the last '.', but does not treat a '.'
# that precedes a directory separator as an extension.
BN := $(basename $(LIST))

all:
	@echo dir=[$(DR)]
	@echo basename=[$(BN)]
