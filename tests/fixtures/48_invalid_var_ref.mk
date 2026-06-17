# Exercises the invalid-variable-reference diagnostic (variable.rs
# emit_var_name_warning, owned message buffer freed via RAII): a reference
# whose name contains an unquoted blank is invalid, so --warn=invalid-ref must
# emit the identical warning text from both binaries.
all:
	@echo got $(foo bar)
