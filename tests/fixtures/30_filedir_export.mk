# Exercises eval's directive dispatch now that the file-directive arms classify
# through the interned whole-line node (`classify_line`): the bare-word `export`
# arm must still win for `export`, while `vpath` and the optional include forms
# are recognised by the unified line classifier.
export EXPORTED = from-parent
vpath %.y gen
-include /nonexistent-makers-test-c.mk

PLAIN = plain-value
all:
	@echo exported=$(EXPORTED)
	@echo plain=$(PLAIN)
