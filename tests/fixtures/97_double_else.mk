ifeq (a,a)
all: ; @echo one
else
all: ; @echo two
else
all: ; @echo three
endif
