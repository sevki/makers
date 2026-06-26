# `$(wildcard)` drives dir_contents_file_exists_p. The makefile's own path
# (from MAKEFILE_LIST) is scanned out of its directory (found case); a bogus
# sibling name exercises the not-found case. Output is deterministic (basename
# only) and compared against the C oracle.
MK := $(firstword $(MAKEFILE_LIST))
DIR := $(dir $(MK))
FOUND := $(notdir $(wildcard $(MK)))
MISS := $(wildcard $(DIR)no_such_sibling_zzz.mk)
all: ; @echo found=$(FOUND) miss=$(MISS)
