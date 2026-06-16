# Companion to 39: a normal first target runs the suffix-rule check loop
# without matching, so it IS auto-selected as .DEFAULT_GOAL.
.SUFFIXES: .c .o
hello:
	@echo hello
show:
	@echo goal=[$(.DEFAULT_GOAL)]
