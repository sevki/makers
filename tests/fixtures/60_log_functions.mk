# Exercises the $(error)/$(warning)/$(info) diagnostic functions, all of which
# route through the single func_error handler now classified via the typed
# LogFunction AST classifier.

# $(info) prints to stdout with a trailing newline; $(warning) prints a
# located diagnostic to stderr; neither stops the build.
$(info info line $(words a b c))
$(warning warning line)

ifdef BOOM
# $(error) prints a located diagnostic to stderr and stops the build.
$(error fatal line)
endif

all:
	@echo built
