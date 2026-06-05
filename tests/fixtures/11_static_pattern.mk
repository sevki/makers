# Static pattern rule whose targets ($(OBJS)) are also the prerequisites of a
# later `all` rule. Note the GNU make gotcha exercised by the tests in
# tests/rs_integration.rs: the first static-pattern target (x1.o) becomes the
# default goal, so a bare `make` builds only x1.o; `make all` builds all three.
OBJS = x1.o x2.o x3.o

$(OBJS): %.o: %.c
	@echo build $@

all: $(OBJS)
