A := 1
S1 := $(words $(filter A B C,$(.VARIABLES)))
B := 2
S2 := $(words $(filter A B C,$(.VARIABLES)))
C := 3
S3 := $(words $(filter A B C,$(.VARIABLES)))

all:
	@echo S1=$(S1)
	@echo S2=$(S2)
	@echo S3=$(S3)
