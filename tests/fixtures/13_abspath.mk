# Absolute-path inputs only, so $(abspath ...) is independent of the working
# directory and the C oracle / Rust outputs compare byte-for-byte.
all:
	@echo collapse: $(abspath /usr//lib/../bin)
	@echo dot: $(abspath /usr/./bin)
	@echo trail: $(abspath /usr/bin/)
	@echo root-dotdot: $(abspath /../usr)
	@echo root: $(abspath /)
	@echo multi: $(abspath /a/b/c/.. /tmp/./x /p/q/../../r)
