# Exercises the .POSIX special target (posix_pedantic): declaring it sets the
# pedantic flag, after which make follows POSIX-conformant behavior. The build
# itself is ordinary; the point is that the special-target path that sets the
# flag is taken and the makefile still behaves identically to the C oracle.
.POSIX:

GREETING = hi

all:
	@echo posix-$(GREETING)
