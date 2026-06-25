# Exercises `-O`/`--output-sync`, whose resolved mode now lives in
# `Options::output_sync` (the former `static mut output_sync`), fed from the
# already-owned `output_sync_option` by `decode_output_sync_flags`. For a
# non-parallel build the mode is gated back to none, so the output matches a
# plain build — the point is that the decode / `syncing` / per-job `set_syncout`
# paths stay byte-for-byte matched to the C oracle across the flag-set and
# flag-clear states.
all: a b
	@echo done $@

a:
	@echo build $@

b:
	@echo build $@
