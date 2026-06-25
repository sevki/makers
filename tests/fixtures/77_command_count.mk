# Exercises COMMAND_COUNT (the former `static mut command_count`): the directory
# cache tags its stat/contents entries with the command-generation counter and
# invalidates them once a recipe command has run, so a file created by an earlier
# recipe becomes visible to a later `$(wildcard)`. `probe` stats the (empty)
# directory and caches it; `gen` creates made.tmp (bumping COMMAND_COUNT when the
# command completes); `show`'s `$(wildcard *.tmp)` must then re-read the directory
# and see made.tmp. Output is deterministic and matched against the C oracle.
all: probe gen show

probe:
	@echo probe $(wildcard *.tmp)

gen:
	@touch made.tmp

show:
	@echo show $(wildcard *.tmp)
