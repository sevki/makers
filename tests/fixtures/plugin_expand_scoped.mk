# Fixture for the seam between the two expanders.
#
# `expand_recipe_line` substitutes in the *target's* scope but leaves a
# balanced function call alone, so `$(addprefix -l,$(LIBS))` reaches
# `vars.expand` with its argument never looked at — and `vars.expand` runs in
# make's global scope by design. `debug.o` overrides `LIBS`, so expanding
# that recipe globally yields `-lm -lpthread` where make itself uses
# `-ldebug`: a wrong answer that looks like a right one.
CC := cc
CFLAGS := -Wall
LIBS := m pthread

all: lib.o debug.o

debug.o: LIBS := debug

lib.o: lib.c
	$(CC) $(CFLAGS) -c -o $@ $< $(addprefix -l,$(LIBS))

debug.o: debug.c
	$(CC) $(CFLAGS) -c -o $@ $< $(addprefix -l,$(LIBS))
