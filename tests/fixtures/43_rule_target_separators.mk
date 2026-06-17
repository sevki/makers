# Exercises eval's rule-target parser, the path now routed through
# bounds-checked `variable_buffer` byte accessors instead of raw-pointer
# dereferences of the cursors `find_char_unquote` returns:
#   - an inline `;` recipe on the target line (the `cmdleft` write),
#   - the `:` target/prereq separator (the `colonp` save/restore), and
#   - `&:` grouped targets (the `&` look-back and `also_make_targets`).
# Both binaries must parse all three identically.
all: grouped inline

inline: ; @echo inline-recipe

g1 g2 &:
	@echo grouped $@

grouped: g1 g2
	@echo grouped-done
