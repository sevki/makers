# Exercises the `else <directive>` chain path in conditional_line: an `else`
# immediately followed by a fresh conditional directive (else ifeq / else ifdef),
# plus a final plain else and nested conditionals.
ifeq ($(MODE),debug)
TAG = dbg
else ifeq ($(MODE),release)
TAG = rel
else ifdef FALLBACK
TAG = fb-$(FALLBACK)
else
TAG = none
endif

ifndef HIDE
SHOWN = yes
else
SHOWN = no
endif

all:
	@echo tag=$(TAG) shown=$(SHOWN)
