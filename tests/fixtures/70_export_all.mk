# Exercises a bare `export` directive / `export_all_variables`, now resolved
# through `Options::export_all_variables` instead of the former
# `static mut export_all_variables`. With export-all on, every exportable make
# variable is placed in each recipe's environment, so the recipe's `$$FOO`
# (a shell variable reference) sees the make variable's value. The `@` keeps the
# recipe line itself out of the output. Differential-checked against the C
# oracle; covers the `export`-directive writer in `read::eval`.
FOO = exported-value
export

all:
	@echo "FOO=[$$FOO]"
