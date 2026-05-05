# Triggers: undefined-var (referencing $(MISSING)), invalid-var.
# The --warn flag in the test args picks the policy.

NAME = $(MISSING)

all:
	@echo using $(NAME) and $(ALSO_MISSING)
