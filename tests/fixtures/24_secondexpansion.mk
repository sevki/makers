# Exercises the .SECONDEXPANSION flag (second_expansion): after the special
# target is declared, prerequisites are expanded a second time, so `$$(VAR)`
# in a prereq list resolves on the second pass. Without second expansion the
# `$$` would just collapse to a literal `$`.
.SECONDEXPANSION:

DEP = real-dep

all: $$(DEP)
	@echo built-all

real-dep:
	@echo made-$@
