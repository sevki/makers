# Regression fixture: a static pattern rule whose targets ($(OBJS)) are the
# prerequisites of the default goal. GNU make builds all of them; see the
# `static_pattern_default_goal` test in tests/rs_integration.rs.
OBJS = x1.o x2.o x3.o

$(OBJS): %.o: %.c
	@echo build $@

all: $(OBJS)
