# Exercises the `load` directive stub (load.rs load_file, now routed through
# the safe `fatal!` macro): dynamic loading is unsupported in this port, so
# `load` must abort with the identical "'load' is not supported" diagnostic
# from both binaries.
load foo.so

all:
	@echo unreached
