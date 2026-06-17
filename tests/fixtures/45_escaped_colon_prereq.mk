# Exercises unescape_char on a prerequisite string: `\:` is an escaped colon
# (a literal `:` in a file name, not the rule's target/prereq separator). Both
# binaries must unescape `foo\:bar` to the prerequisite `foo:bar` and build the
# matching escaped-colon target.
all: foo\:bar

foo\:bar:
	@echo built $@
