# Exercises conditional_line's ifeq/ifneq argument forms now parsed through the
# typed AST layer: the paren form with embedded references and spaces, the
# double- and single-quoted forms, and a balanced-parenthesis second argument.
A = foo
B = foo
C = bar
NEST = x(y)

# Paren form with variable references on both sides (equal).
ifeq ($(A),$(B))
R1 = eq
else
R1 = ne
endif

# Paren form, unequal, with surrounding spaces (whitespace is significant in
# make's exact scan, so this is "ne").
ifeq ( $(A) , $(C) )
R2 = eq
else
R2 = ne
endif

# Double-quoted form.
ifeq "$(A)" "foo"
R3 = eq
else
R3 = ne
endif

# Single-quoted form, ifneq.
ifneq '$(A)' '$(C)'
R4 = differ
else
R4 = same
endif

# Balanced parentheses inside the second argument.
ifeq ($(NEST),x(y))
R5 = eq
else
R5 = ne
endif

all:
	@echo $(R1) $(R2) $(R3) $(R4) $(R5)
