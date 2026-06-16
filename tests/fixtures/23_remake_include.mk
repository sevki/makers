# Exercises the makefile-remaking goal-chain pass (rebuilding_makefiles): the
# included `gen.mk` does not exist yet, so make runs its rule to generate it,
# sets rebuilding_makefiles during that pass, then re-reads the makefiles with
# the now-defined variable available.
include gen.mk

gen.mk:
	@echo 'GEN = generated-value' > gen.mk

all:
	@echo val=$(GEN)
