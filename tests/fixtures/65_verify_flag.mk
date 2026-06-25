# Exercises the always-on maintainer-mode file-database verification that the
# former `verify_flag` global gates. It is now owned on `main_0`'s `Options`
# (`Options::verify`) and read through the `with_options` borrow channel instead
# of a `static mut`; in this (maintainer) build it is set unconditionally at
# startup, so every `enter_file` asserts its name is strcache-interned and
# `verify_file_data_base` walks the whole file graph when the run finishes. A
# small diamond (two targets sharing a prerequisite) plus a .PHONY goal
# populates a non-trivial graph; serial output is byte-stable, so we compare it
# directly against the C oracle.

.PHONY: all
all: left right
	@echo done all

left: shared
	@echo done left

right: shared
	@echo done right

shared:
	@echo done shared
