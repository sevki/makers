NAME = world
GREETING := hello $(NAME)
LATE = lazy $(NAME)
NAME = mars

all:
	@echo $(GREETING)
	@echo $(LATE)
	@echo override-from-cmd: $(FROMCMD)
