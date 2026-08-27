# Fixture for the `expand-variables` capability.
#
# The recipe uses a makefile *function*, which target-scoped substitution
# cannot resolve on its own: `$(addprefix ...)` is not a variable lookup. A
# plugin therefore either skips the entry or asks the host to finish the
# expansion with make's real expander — which is exactly the trade the
# capability exists to make explicit, since that same expander can run
# `$(shell ...)`.
CC := cc
CFLAGS := -Wall
LIBS := m pthread

lib.o: lib.c
	$(CC) $(CFLAGS) -c -o $@ $< $(addprefix -l,$(LIBS))
