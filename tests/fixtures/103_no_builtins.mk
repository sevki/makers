# Drives `disable_builtins` and the two halves it dispatches to —
# `clear_builtin_rules` and `undefine_default_variables` — neither of which had
# a fixture before. Since #442 both return `Result`, because the definitions
# and undefinitions they perform can now be refused rather than exiting.
#
# `-r` clears the built-in rules and suffix list but keeps the default
# variables; `-R` additionally undefines those variables, so `$(CC)` is empty
# under `-R` and `cc` under `-r`.
all:
	@echo "CC=[$(CC)] AR=[$(AR)] suffixes=[$(.SUFFIXES)]"
