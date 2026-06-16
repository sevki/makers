# Exercises eval's leading-byte line classification (LineKind): a custom
# `.RECIPEPREFIX` makes `>` the recipe character, so recipe lines begin with
# `>` rather than a tab. Blank lines are skipped; non-prefixed lines are parsed
# as normal makefile syntax (variables, rules).
.RECIPEPREFIX = >

GREETING = hello

all: a b

a:
> @echo a-$(GREETING)

b:
> @echo b-$(GREETING)
> @echo b-done
