# Exercises check_special_file (now using the parser::is_wait_token helper):
# a `.WAIT` target with prerequisites must trigger the
# ".WAIT should not have prerequisites" diagnostic identically in both binaries.
.WAIT: foo

all:
	@echo hi

foo:
	@echo foo
