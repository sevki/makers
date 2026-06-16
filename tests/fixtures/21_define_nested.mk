# Exercises the define/endef block scanner: a nested `define` inside a define
# body (the reader nests `nlevels` on the inner define and unwinds on each
# `endef`), an `endef` followed by a trailing comment, and a `define` that is
# skipped inside a false conditional (the `in_ignored_define` path).

define OUTER
outer-before
define INNER
inner-body
endef
outer-after
endef # trailing comment after endef

ifdef NOT_SET
define SKIPPED
this body is ignored
endef
endif

all:
	@echo outer=[$(OUTER)]
	@echo inner=[$(INNER)]
	@echo skipped=[$(SKIPPED)]
