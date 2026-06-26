# `$(wildcard)` drives dir_contents_file_exists_p's directory scan. Two files
# are created at parse time (before any wildcard), so both make binaries scan
# the same populated cwd and the glob match path runs deterministically. The
# count (not the names) is compared, so it is order- and load-independent.
$(shell : >a1.tmp; : >a2.tmp)
COUNT := $(words $(wildcard *.tmp))
MISS := $(wildcard no_such_sibling_zzz.tmp)
all: ; @echo count=$(COUNT) miss=$(MISS)
