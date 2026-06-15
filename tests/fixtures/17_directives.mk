# Exercises eval's file-directive classification: vpath, and the error-tolerant
# include forms (-include / sinclude) of files that do not exist (silently
# ignored).
vpath %.x src
-include /nonexistent-makers-test-a.mk
sinclude /nonexistent-makers-test-b.mk

GREETING = hello-from-directives
all:
	@echo $(GREETING)
