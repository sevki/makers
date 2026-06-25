# Exercises `-B`/`--always-make`, now resolved into `ExecContext::always_make_flag`
# (read by `update_file_1`/`set_file_variables`) instead of the former
# `static mut always_make_flag` global. On a fresh build every target builds
# regardless, so `-B` and plain make produce the same output here — the point is
# that both the flag-set (`-B`) and flag-clear paths stay byte-identical to the C
# oracle. The `-B` force-rebuild-of-an-up-to-date-target behavior is covered
# separately by the Rust-only `always_make_rebuilds_up_to_date_target` test.
all: a b
	@echo done $@

a:
	@echo build $@

b:
	@echo build $@
