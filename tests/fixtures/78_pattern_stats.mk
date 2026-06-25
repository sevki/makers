# Exercises the pattern-rule statistics now owned by `ExecContext`
# (num_pattern_rules / max_pattern_targets / max_pattern_deps /
# max_pattern_dep_length): `snap_implicit_rules` computes them from the rule set
# and `pattern_search` reads them to size its scratch buffers. The `%.out` rule
# has three prerequisites, so resolving `widget.out` runs the deps/length/count
# bookkeeping the stats track. Output is deterministic (a serial, left-to-right
# build of a fixed dependency tree) and matched byte-for-byte against the C oracle.
%.out: %.a %.b %.c
	@echo out $@ from $^

widget.a:
	@echo made $@

widget.b:
	@echo made $@

widget.c:
	@echo made $@
