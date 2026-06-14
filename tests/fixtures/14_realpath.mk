# Absolute, always-present inputs so $(realpath ...) is independent of the
# working directory; the C oracle and Rust port resolve them identically
# (both delegate to libc realpath). A non-existent path is dropped by both.
all:
	@echo root: $(realpath /)
	@echo dots: $(realpath /usr/./.. /usr)
	@echo missing: [$(realpath /no/such/makers_xyzzy)]
