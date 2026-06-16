# Exercises parse_file_seq token normalization (now via pure parser helpers):
# the `.WAIT` ordering marker between prerequisites (PARSEFS_WAIT) and the
# stripping of redundant `./` prefixes from target/prereq names. Both binaries
# must normalize `./c` to `c` and honor `.WAIT` identically.
all: a .WAIT b ./c

a:
	@echo a
b:
	@echo b
c:
	@echo c
