# $(subst) and the no-% $(patsubst) path both run through subst_expand.
# All outputs are deterministic and working-directory independent, so the C
# oracle and the Rust port compare byte-for-byte.
all:
	@echo multi: $(subst a,X,banana)
	@echo empty-replace: [$(subst a,,banana)]
	@echo no-match: $(subst z,X,banana)
	@echo at-start: $(subst ba,X,banana)
	@echo at-end: $(subst na,X,banana)
	@echo non-overlap: $(subst aa,X,aaaa)
	@echo whole: $(subst banana,X,banana)
	@echo longer-replace: $(subst a,YY,banana)
	@echo word-match: $(patsubst foo,bar,foo foobar barfoo foo)
	@echo word-empty: [$(patsubst foo,,foo foobar foo)]
