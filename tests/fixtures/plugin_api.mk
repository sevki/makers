# Fixture for the build-plugin end-to-end tests (tests/plugin_*.rs).
#
# Exercises the parts of the graph the `makers:plugin` interface exposes and
# a plugin has to get right: variables resolved through an implicit rule, a
# per-target override that must beat the global value, an order-only
# prerequisite that is *not* a compile input, a link step that must not be
# mistaken for a compile, and a phony target.
CC := cc
CFLAGS := -Wall
OBJS := main.o util.o debug.o

prog: $(OBJS) | build
	$(CC) $(CFLAGS) -o $@ $(OBJS)

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

# Per-target override: `debug.o` must come out with these flags, not the
# global `-Wall`. Reading the global value here is the classic
# compile-database bug.
debug.o: CFLAGS := -Wall -O0 -g

.PHONY: build
build:
	mkdir -p build
