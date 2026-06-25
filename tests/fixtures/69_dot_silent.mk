# A bare `.SILENT` target silences every recipe for the run, exercising the
# `snap_deps` writer of `run_silent` (the former `run_silent = 1`, now
# `Options::run_silent`). Unlike `-s`, `.SILENT` is set from makefile content,
# so this also covers the makefile-time write path. Output must match the C
# oracle byte for byte.
.SILENT:

all:
	echo building $@
	echo done $@
