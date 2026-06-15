# Exercises func_shell and its shell_function_completed spin-loop: $(shell ...)
# runs a child, then func_shell waits for the reaper callback to set the
# completion flag. Output is deterministic.
SHELLOUT := $(shell echo hello-from-shell)
WORDS := $(shell printf 'a\nb\nc\n')
all:
	@echo out=[$(SHELLOUT)]
	@echo words=[$(WORDS)]
