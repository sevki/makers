# Strict `include` of a missing file: make must fail (the error path of the
# include directive), identically to the C oracle.
include /nonexistent-makers-test-strict.mk
all:
	@echo unreachable
