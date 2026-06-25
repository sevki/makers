# Exercises a bare `.NOTINTERMEDIATE`, which sets the per-run `no_intermediates`
# latch (now `ExecContext::no_intermediates`, the twin of
# `ExecContext::all_secondary`, replacing two process-global statics). It marks
# every file non-intermediate, so make does NOT auto-delete the pattern-rule-built
# intermediate `foo.mid` after `foo.out` — i.e. no `rm foo.mid` line is emitted.
# If the latch were misread, make would delete the intermediate and the output
# would diverge from the C oracle. Serial output is byte-stable, so compare
# directly.
.NOTINTERMEDIATE:

all: foo.out

%.out: %.mid
	@echo out $@
	@: > $@

%.mid:
	@echo mid $@
	@: > $@
