# Exercises `-lNAME` prerequisite resolution (`library_search` /
# `.LIBPATTERNS`): a library-style prerequisite is resolved against the
# pattern list, first checking the plain relative name, then vpath, then
# falling back to the fixed system directories (/lib, /usr/lib,
# /usr/local/lib) — the branch that populates and reuses the search-path
# cache. `libfoo.a` is created at parse time (via `$(shell)`) so it exists
# before the prerequisite scan runs.
DUMMY := $(shell touch libfoo.a)
.LIBPATTERNS = lib%.a

prog: -lfoo
	@echo built $@ from $<

all: prog missing

missing: -lthislibrarydoesnotexistanywhere987
	@echo unexpectedly-built $@
