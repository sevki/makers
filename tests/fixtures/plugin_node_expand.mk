# Fixture for node.expand and node.recipe-environment.
#
# `mk` builds its command from the automatic variables with a make function,
# which is the shape that cannot be expanded before `$@` and `$<` are known.
# KBUILD_FLAG is exported, so make puts it in the recipe's environment.
export KBUILD_FLAG := from-make
mk = echo built $(1) from $< and $^

all: out.o other.o

out.o: in.c extra.h
	$(call mk,$(@F))

# A per-target variable must win over the global one in the expansion.
other.o: in.c
other.o: WHO := target
other.o:
	echo $(WHO) $@

WHO := global
