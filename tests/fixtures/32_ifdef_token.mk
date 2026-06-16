# Exercises conditional_line's ifdef/ifndef single-token extraction (now via the
# typed AST layer): a defined and an undefined variable, plus trailing
# whitespace after the variable name (which must be ignored, not error).
DEFINED = yes

ifdef DEFINED
A = def
else
A = undef
endif

ifndef MISSING
B = absent
else
B = present
endif

# A name produced by expansion ($(NAME) → DEFINED) still resolves to one token.
NAME = DEFINED
ifdef $(NAME)
C = expanded-ok
else
C = no
endif

all:
	@echo $(A) $(B) $(C)
