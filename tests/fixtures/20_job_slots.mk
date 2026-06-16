# Exercises job-slot accounting: with -j N and more independent recipes than
# slots, make repeatedly increments job_slots_used on spawn and decrements it
# on reap, blocking once every slot is taken. The set of emitted lines is
# deterministic even though their interleaving is not.
.PHONY: all j1 j2 j3 j4 j5 j6

all: j1 j2 j3 j4 j5 j6
	@echo all-done

j1:; @echo slot-1
j2:; @echo slot-2
j3:; @echo slot-3
j4:; @echo slot-4
j5:; @echo slot-5
j6:; @echo slot-6
