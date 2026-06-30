# Pattern-specific (target-specific on a `%` stem) variables must be visible
# when the matching target's recipe is expanded. Regression test for the
# FileId/FileNode flip, where the pattern's stored target/suffix pointers
# dangled and `lookup_pattern_var` failed to match at build time — silently
# dropping flags like the kernel's `$(obj)/%.so: OBJCOPYFLAGS := ...`.

obj := build

# Pattern-specific var with a directory prefix (mirrors the kernel vdso rule).
$(obj)/%.so: OBJCOPYFLAGS := -S --remove-section __ex_table
$(obj)/%.so: $(obj)/%.so.dbg
	@echo 'SO   flags=[$(OBJCOPYFLAGS)] stem=$* target=$@'

# Plain pattern var, plus an append, plus an exact-target override.
%.q: PV := base
%.q: PV += more
%.q:
	@echo 'Q    PV=[$(PV)] target=$@'

exact.q: PV := overridden
exact.q:
	@echo 'EX   PV=[$(PV)] target=$@'

all: $(obj)/vdso64.so a.q exact.q
	@echo done

$(obj)/%.so.dbg:
	@mkdir -p $(obj) && echo dbg > $@

.PHONY: all
