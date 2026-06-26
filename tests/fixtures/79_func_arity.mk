# Calling a builtin function with too few arguments drives
# expand_builtin_function's "insufficient number of arguments" fatal path
# (subst needs 3 args; only 2 are given). Both binaries must abort identically.
X := $(subst a,b)
all: ; @echo $(X)
