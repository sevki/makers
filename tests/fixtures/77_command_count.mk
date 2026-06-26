# Exercises COMMAND_COUNT (the former `static mut command_count`): the directory
# cache tags its stat/contents entries with the command-generation counter and
# invalidates them once a recipe command has run, so a file created by an earlier
# recipe becomes visible to a later `$(wildcard)`. `probe` stats the (empty)
# directory and caches it; `gen` creates made.tmp (bumping COMMAND_COUNT when the
# command completes); `show`'s `$(wildcard *.tmp)` must then re-read the directory
# and see made.tmp. Output is deterministic and matched against the C oracle.
#
# The visibility invariant only holds for a serial build, so the three steps form
# an explicit probe -> gen -> show chain: even under an inherited jobserver they
# execute in order, and `show` always samples the directory after `gen`'s touch.
all: show

probe:
	@echo probe $(wildcard *.tmp)

gen: probe
	@touch made.tmp

show: gen
	@echo show $(wildcard *.tmp)
