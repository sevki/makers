# Exercises eval's "ifeq/ifneq must be followed by whitespace" diagnostic (now
# classified in the typed AST layer). The recipe-less line `ifeq(a,a)` has no
# separator after the keyword, so make must report the *specific* error rather
# than the generic "missing separator". Both binaries must match byte-for-byte.
ifeq(a,a)
X = y
endif

all:
	@echo $(X)
