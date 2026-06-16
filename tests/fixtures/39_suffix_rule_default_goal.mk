# Exercises check_specials default-goal suffix-rule rejection (now comparing
# names as CStr byte slices): `ab` = suffix `a` + suffix `b` is a suffix rule,
# so it is NOT auto-selected as .DEFAULT_GOAL -- selection falls through to the
# next normal target (`show`). If the rejection were broken the goal would be
# `ab`. Both binaries must print the identical goal.
.SUFFIXES:
.SUFFIXES: a b
ab:
	@echo hi
show:
	@echo goal=[$(.DEFAULT_GOAL)]
