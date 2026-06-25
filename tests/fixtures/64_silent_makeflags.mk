# Echo MAKEFLAGS so the presence of `-s` — which flips silent_flag away from
# its immutable `default_silent_flag` default — is observable through
# define_makeflags' flag-vs-default comparison.
all:
	echo "MAKEFLAGS=[$(MAKEFLAGS)]"
