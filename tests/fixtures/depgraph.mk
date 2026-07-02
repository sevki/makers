# Fixture for the MAKERS_DEPGRAPH end-to-end test (tests/depgraph_dump.rs):
# a real makefile exercising an explicit link rule, a user-defined pattern
# rule, and an order-only prerequisite on a phony target.
prog: main.o util.o | outdir
	cc -o $@ main.o util.o

%.o: %.c
	cc -c -o $@ $<

.PHONY: outdir
outdir:
	mkdir -p outdir
