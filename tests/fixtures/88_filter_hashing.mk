# Exercises func_filter_filterout's literal-pattern hashing fast path — now an
# FxHashMap keyed by word content (value = head of the same-content `chain`),
# replacing the c2rust gnulib hash_table + a_word_hash_1/2/cmp callbacks. The
# hash path engages only when there is more than one literal pattern and
# literals * word_count >= 10, so use 5 literal patterns over 10 words. Words
# repeat (a, b, c) so the chain is built and walked when a literal pattern
# matches. Byte-for-byte vs the C oracle for both $(filter) and $(filter-out).
WORDS := a x b y c z a b c d
PATS := a b c d e
all:
	@echo filter=$(filter $(PATS),$(WORDS))
	@echo filterout=$(filter-out $(PATS),$(WORDS))
