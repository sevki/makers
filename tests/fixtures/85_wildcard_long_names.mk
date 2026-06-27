# Drives read_dirstream's reused dirent scratch buffer (the former process-
# global `static mut buf`/`bufsz`, now per-run on ExecContext, reached by the
# glob `gl_readdir` callback through the CTX_PTR borrow channel). A single
# $(wildcard) over one directory enumerates files whose names vary widely in
# length, so read_dirstream must grow the dirent buffer whenever a longer name
# follows a shorter one — exercising the realloc path. The files are created at
# parse time before the wildcard, so both binaries scan an identical tree;
# $(sort) makes the compared output order- and load-independent.
$(shell mkdir -p wbuf; for n in a bb ccc dddd eeeeeeee ffffffffffffffff gggggggggggggggggggggggggggggggg; do : >wbuf/$$n.x; done)
NAMES := $(sort $(notdir $(wildcard wbuf/*.x)))
all: ; @echo names=$(NAMES) count=$(words $(NAMES))
