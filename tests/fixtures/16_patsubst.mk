# The %-pattern path of $(patsubst) runs through patsubst_expand_pat.
# Deterministic, working-directory independent — compared byte-for-byte
# against the C oracle.
all:
	@echo suffix: $(patsubst %.c,%.o,a.c b.c d.h)
	@echo pre-suf: $(patsubst src/%.c,obj/%.o,src/a.c src/b.c x)
	@echo no-pct-replace: $(patsubst %.c,X,a.c b.c)
	@echo prefix-only: $(patsubst pre%,POST,prefix preXY nope)
	@echo suffix-only: $(patsubst %suf,%END,asuf bsuf nope)
	@echo mid-pct: $(patsubst a%c,X%Z,abc aXc ac)
	@echo whole: $(patsubst %,P%S,x yy)
	@echo no-match: $(patsubst %.zz,%.qq,a.c b.h)
