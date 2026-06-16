# Exercises record_files's second-expansion prereq check (now via the
# parser::prereq_needs_second_expansion helper): under .SECONDEXPANSION a prereq
# containing `$$` is re-expanded, so `all` ends up depending on dep1.
.SECONDEXPANSION:
VAR = dep1
all: $$(VAR)
	@echo built $@ from $^
dep1:
	@echo made dep1
