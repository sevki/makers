# Exercises the .ONESHELL special target (recognised by the SpecialTarget
# classifier in check_specials): with .ONESHELL all recipe lines of a target run
# in a single shell, so a variable set on one line is visible on the next. Only
# the first line's `@` prefix is honored for the whole recipe.
.ONESHELL:

all:
	@v=oneshell
	echo $$v
