# Drives the directory cache on both sides of the MAKELEVEL context rebuild:
# `$(wildcard)` at parse time populates the cache (via the glob `open_dirstream`
# callback, which now reaches the per-run cache through the borrow channel), and
# `$(wildcard)` again in the recipe re-reads it after the build-phase rebuild,
# which must hand the cache across. Both files are created at parse time before
# either wildcard, so both make binaries scan an identical tree; only the match
# counts are compared, so the result is order- and load-independent.
$(shell : >w1.tmp; : >w2.tmp)
PARSE := $(words $(wildcard *.tmp))
all: ; @echo parse=$(PARSE) build=$(words $(wildcard *.tmp))
