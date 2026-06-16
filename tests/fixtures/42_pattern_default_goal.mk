# Exercises check_specials default-goal pattern check (now via CStr byte
# slice): a `%`-pattern first target is never auto-selected as .DEFAULT_GOAL.
%.o:
	@echo pattern
show:
	@echo goal=[$(.DEFAULT_GOAL)]
