# Exercises `$(origin NAME)` and `$(flavor NAME)` across the origin/flavor
# combinations reachable from a plain invocation: undefined, file-defined
# (recursive and simple), override directive, command-line variable, an
# environment variable, and an automatic variable inside a recipe.
# `CMDLINE_VAR` is supplied as a command-line variable definition and
# `ORIGIN_ENV_VAR` as a process environment variable by the test driver.
FILE_RECURSIVE = value
FILE_SIMPLE := value
override OVERRIDE_VAR = value

all:
	@echo undefined-origin=$(origin NOPE)
	@echo undefined-flavor=$(flavor NOPE)
	@echo file-recursive-origin=$(origin FILE_RECURSIVE)
	@echo file-recursive-flavor=$(flavor FILE_RECURSIVE)
	@echo file-simple-origin=$(origin FILE_SIMPLE)
	@echo file-simple-flavor=$(flavor FILE_SIMPLE)
	@echo override-origin=$(origin OVERRIDE_VAR)
	@echo cmdline-origin=$(origin CMDLINE_VAR)
	@echo cmdline-flavor=$(flavor CMDLINE_VAR)
	@echo env-origin=$(origin ORIGIN_ENV_VAR)
	@echo env-flavor=$(flavor ORIGIN_ENV_VAR)
	@echo automatic-origin=$(origin @)
	@echo automatic-flavor=$(flavor @)
