WORDS := alpha beta gamma delta
PATHS := src/foo.c src/bar.c lib/baz.c

all:
	@echo subst: $(subst beta,BETA,$(WORDS))
	@echo patsubst: $(patsubst %.c,%.o,$(PATHS))
	@echo filter: $(filter src/%,$(PATHS))
	@echo filter-out: $(filter-out lib/%,$(PATHS))
	@echo words: $(words $(WORDS))
	@echo word-2: $(word 2,$(WORDS))
	@echo firstword: $(firstword $(WORDS))
	@echo lastword: $(lastword $(WORDS))
	@echo dir: $(dir $(PATHS))
	@echo notdir: $(notdir $(PATHS))
	@echo basename: $(basename src/foo.tar.gz)
	@echo suffix: $(suffix src/foo.tar.gz)
	@echo if-true: $(if true,yes,no)
	@echo if-false: $(if ,yes,no)
	@echo strip: '$(strip   spaced   out  )'
	@echo upper: $(shell echo hello | tr a-z A-Z)
