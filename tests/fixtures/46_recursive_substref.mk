# Exercises the recursive-variable + substitution-reference expansion path in
# expand.rs (recursively_expand_for_file, freed via RAII OwnedCStr): SRC is a
# recursive (`=`) variable, and `$(SRC:.c=.o)` rewrites each `.c` word to `.o`.
# Also covers a recursive value flowing through a plain `$(...)` reference.
SRC = a.c b.c c.c
OBJ = $(SRC:.c=.o)

all:
	@echo objs $(OBJ)
	@echo src $(SRC)
