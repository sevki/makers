# Exercises the variable-modifier classification in eval: export, unexport,
# override, private, define/endef, and undefine.
override OVR = from-makefile
export EXP = exported-value
unexport EXP2
private PRIV = private-value

define MULTI
line-one
line-two
endef

UNDEF = will-be-removed
undefine UNDEF

all:
	@echo ovr=$(OVR)
	@echo priv=$(PRIV)
	@echo multi=[$(MULTI)]
	@echo undef=[$(UNDEF)]
	@echo exp-in-env=[$${EXP:-unset}]
