# A pattern-rule target reached as an *intermediate* prerequisite of another
# pattern rule must still expose its prerequisites in the automatic variables.
# Regression test for the FileId/FileNode flip: `merge_intermediate` clears an
# intermediate dep's `name` (relying on its file handle, like C make), but the
# dep-name accessor lacked C's `dep_name()` fallback to `file->name`, so `$^`
# and `$<` came out empty for such targets. This is the kernel's
# `vdso-image-%.c: $(obj)/vdso%.so` -> `$(obj)/%.so: %.so.dbg FORCE` chain,
# which left objcopy with no input file and failed the vdso64.so build.

all: a-64.c

a-%.c: b-%.so FORCE
	@echo 'A $@: [$^] first=[$<]'
	@echo y > $@

b-%.so: FORCE
	@echo 'B $@: [$^] first=[$<]'
	@echo y > $@

FORCE:
.PHONY: FORCE all
