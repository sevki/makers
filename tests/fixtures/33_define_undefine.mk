# Exercises do_define / do_undefine name isolation (now via the typed AST
# layer): a define whose name comes from expansion, and an undefine that removes
# a previously-set variable.
WHICH = GREETING

define $(WHICH)
hello-world
endef

KEEP = stays
undefine KEEP

all:
	@echo greeting=[$(GREETING)]
	@echo keep=[$(KEEP)]
