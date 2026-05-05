SOURCES = a.c b.c c.c
OBJECTS = $(SOURCES:.c=.o)

all: $(OBJECTS)
	@echo done

%.o: %.c
	@echo '  CC $<'
	@echo '  -> $@'

a.c b.c c.c:
	@echo '  touch $@' && touch $@

.PHONY: all
