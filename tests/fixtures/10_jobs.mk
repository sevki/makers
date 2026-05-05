# Exercises the jobserver: -j N causes make to set up the pipe/fifo,
# acquire/release tokens around recipe spawns, and tear down at exit.
.PHONY: all a b c d

all: a b c d
	@echo done

a:; @echo a-done
b:; @echo b-done
c:; @echo c-done
d:; @echo d-done
