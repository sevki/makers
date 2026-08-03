# `.EXTRA_PREREQS` — prerequisites added to every target without appearing in
# `$^`/`$?`. Exercises `snap_deps` -> `expand_extra_prereqs`, including the
# expansion of the variable's own value (`$(DEP2)` below) and the per-target
# form, which goes through `expand_extra_prereqs_value` instead.

DEP2 = gen2

.EXTRA_PREREQS = gen1 $(DEP2)

all: real
	@echo all-done

# The extra prerequisites are built, but stay out of the automatic variables:
# `$^` lists only the explicitly written prerequisite.
real: written
	@echo 'real: deps=[$^]'

written:
	@echo built-written

gen1:
	@echo built-gen1

gen2:
	@echo built-gen2

# A per-target `.EXTRA_PREREQS` overrides the global one for this target only.
scoped: .EXTRA_PREREQS = gen3
scoped:
	@echo built-scoped

gen3:
	@echo built-gen3
