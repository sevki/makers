# Exercises parse_file_seq token normalization (now via pure parser helpers):
# the `.WAIT` ordering marker between prerequisites (PARSEFS_WAIT) and the
# stripping of redundant `./` prefixes from target/prereq names. Both binaries
# must normalize `./c` to `c` and honor `.WAIT` identically.
#
# `.WAIT` between every prerequisite fully serializes a -> b -> c so the recipe
# output order is deterministic; with a single `.WAIT` the trailing pair (b, c)
# is unordered and races under parallel builds, which made the differential
# stdout check flaky (C and Rust could each win the race differently).
all: a .WAIT b .WAIT ./c

a:
	@echo a
b:
	@echo b
c:
	@echo c
