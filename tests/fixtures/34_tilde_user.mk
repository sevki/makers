# Exercises tilde_expand's `~user` branch (now slice/CString-based instead of
# strchr + in-place NUL mutation): `~root/<suffix>` is expanded to root's home
# directory + the suffix via getpwnam("root"), then the resulting path is
# included. The file does not exist, so both the C oracle and the Rust port must
# report the *same* expanded path in their "No such file or directory" error.
include ~root/__makers_nonexistent_tilde_user__.mk

all:
	@echo unreachable
