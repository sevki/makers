# A small three-level chain used to drive the query/touch/trace/keep-going
# modes, none of which had a fixture. They exercise large stretches of
# remake.rs and job.rs that this pass's `Result` threading now runs through:
# `-q` answers without building, `-t` touches instead of running recipes,
# `--trace` reports why each target is remade, and `-k` keeps going.
all: out.txt
	@echo built

out.txt: in.txt
	@cp in.txt out.txt

in.txt:
	@echo hello > in.txt
