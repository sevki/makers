# Exercises check_special_file's one-shot `.WAIT` warnings (now AtomicBool
# guards): declaring `.WAIT` as a target with prerequisites and/or commands is
# invalid, and make warns once for each (to stderr) without aborting the build.
all:
	@echo all-done

.WAIT: all
	@echo bogus-recipe
