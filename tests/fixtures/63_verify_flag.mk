# Exercises the file-lookup path that consults `verify_flag()` (now backed by
# the VERIFY_FLAG atomic instead of `static mut verify_flag`). Every
# `enter_file`/`lookup_file` asserts the looked-up name is strcache-interned
# when the maintainer invariant flag is set; this makefile creates several
# named targets and prerequisites so the lookup path runs repeatedly. Output
# is byte-stable, so compare directly against the C oracle.

all: alpha beta gamma
	@echo all done

alpha: beta
	@echo make alpha

beta:
	@echo make beta

gamma: alpha beta
	@echo make gamma
