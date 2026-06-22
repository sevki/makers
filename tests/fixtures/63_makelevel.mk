# Exercises `$(MAKELEVEL)`, now sourced from the safe `execctx::makelevel()`
# accessor backed by the immutable `Config` (replacing `static mut makelevel`).
# At the top level MAKELEVEL is 0; the recursive `$(MAKE)` runs a sub-make whose
# MAKELEVEL is the parent's + 1, exercising both the read and the +1 export.
# Output is byte-stable, so compare directly against the C oracle.

all:
	@echo top=$(MAKELEVEL)
	@$(MAKE) --no-print-directory -f $(firstword $(MAKEFILE_LIST)) sub

sub:
	@echo sub=$(MAKELEVEL)
