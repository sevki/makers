# Exercises `run_silent` (effective recipe-echo suppression), now resolved
# through `Options::run_silent` instead of the former `static mut run_silent`.
# Plain make echoes each recipe line before running it; `-s`/`--silent`
# suppresses that echo (the `decode_switches` writer copies `options.silent`
# into `run_silent`). Both the flag-clear (plain) and flag-set (`-s`) paths must
# stay byte-identical to the C oracle.
all:
	echo building $@
	echo done $@
