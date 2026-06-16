# Exercises func_call's `max_args` save/restore (now an atomic): `$(call)`
# defines $1..$N automatic variables, and max_args tracks the highest index so
# leftover $N from a wider call are cleared on a narrower one. A nested call
# (inner inside outer) exercises the save/restore across recursion.
inner = inner[$(1)][$(2)]
outer = outer[$(1)]:$(call inner,$(1))

all:
	@echo wide=$(call inner,a,b,c)
	@echo narrow=$(call inner,x)
	@echo nested=$(call outer,A,B)
