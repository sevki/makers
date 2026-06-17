FOO := 1
X1 := $(sort $(filter FOO BAR,$(.VARIABLES)))
BAR := 2
X2 := $(sort $(filter FOO BAR,$(.VARIABLES)))

all:
	@echo X1=$(X1)
	@echo X2=$(X2)
