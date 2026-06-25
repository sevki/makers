# Exercises the pattern-rule prerequisite-length bookkeeping that used to live in
# `static mut max_pattern_dep_length` and is now `rule::MAX_PATTERN_DEP_LENGTH`
# (an AtomicUsize, matching the sibling pattern-rule statistics it sits beside).
# The pattern prerequisite name carries a deliberately long suffix, so
# `snap_implicit_rules` records a large value, which `pattern_search` then uses
# to size the scratch buffer it builds the stem-substituted prerequisite name in.
# Building `widget.out` resolves the `%.out` rule; its prerequisite is produced
# by an explicit rule, so the pattern applies. Output is deterministic and is
# matched against the C oracle byte-for-byte.
%.out: %.intermediate_artifact_with_a_deliberately_long_pattern_suffix
	@echo built $@ from $<

widget.intermediate_artifact_with_a_deliberately_long_pattern_suffix:
	@echo built $@
