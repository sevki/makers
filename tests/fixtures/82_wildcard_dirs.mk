# `$(wildcard)` over several sibling subdirectories drives the directory
# cache: each distinct dir scanned opens (and later closes) a `DIR*` stream,
# exercising the open_directories increment/decrement. All files are created
# at parse time before any wildcard, so both binaries scan identical trees.
# Only the match count is compared, so it is order- and load-independent.
$(shell for d in d0 d1 d2 d3 d4; do mkdir -p $$d; : >$$d/f.tmp; done)
COUNT := $(words $(wildcard d0/*.tmp d1/*.tmp d2/*.tmp d3/*.tmp d4/*.tmp))
MISS := $(wildcard d0/no_such_zzz.tmp)
all: ; @echo count=$(COUNT) miss=$(MISS)
