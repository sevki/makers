L := alpha beta gamma delta epsilon

# Success paths: word count, 1-based indexing, $(word N) past the end (empty),
# $(wordlist) clamping its stop index, and an empty result when start is past
# the end.
all:
	@printf 'n=[%s] w3=[%s] first=[%s] last=[%s] wl=[%s] tail=[%s] oob=[%s]\n' '$(words $(L))' '$(word 3,$(L))' '$(word 1,$(L))' '$(word 5,$(L))' '$(wordlist 2,4,$(L))' '$(wordlist 3,99,$(L))' '$(wordlist 9,10,$(L))'

# Fatal-error paths: index 0 trips func_word's `i < 1` guard; a non-numeric
# index trips parse_numeric's validation. Both abort with exit code 2.
badzero:
	@echo $(word 0,a b c)

badnan:
	@echo $(word x,a b c)
