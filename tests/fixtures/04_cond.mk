MODE ?= release

ifeq ($(MODE),debug)
FLAGS = -O0 -g
else
FLAGS = -O2
endif

ifdef VERBOSE
PREFIX = [verbose]
else
PREFIX = [quiet]
endif

all:
	@echo $(PREFIX) flags=$(FLAGS)
ifneq ($(MODE),debug)
	@echo not-debug branch
endif
