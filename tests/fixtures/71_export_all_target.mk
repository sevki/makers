# Exercises the `.EXPORT_ALL_VARIABLES` special target — the `snap_deps` writer
# of `export_all_variables` (former `export_all_variables = 1`, now
# `Options::export_all_variables`). Same observable effect as a bare `export`:
# every exportable make variable enters each recipe's environment, so `$$FOO`
# in the recipe sees the make variable. Output must match the C oracle byte for
# byte.
FOO = target-exported

.EXPORT_ALL_VARIABLES:

all:
	@echo "FOO=[$$FOO]"
