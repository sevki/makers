# Exercises $(MAKELEVEL), now sourced from the threaded ExecContext
# (ctx.makelevel()) instead of the `static mut makelevel` global. The top
# level reports 0; the recursive $(MAKE) sub-make reports the parent's + 1.
# Output is byte-stable, so compare directly against the C oracle.

all:
	@echo top=$(MAKELEVEL)
	@$(MAKE) --no-print-directory -f $(firstword $(MAKEFILE_LIST)) sub

sub:
	@echo sub=$(MAKELEVEL)
