# A parse fatal raised from inside $(eval), not from the makefile body:
# func_eval calls eval_buffer during expansion, so the error unwinds out
# through the expander rather than out of the makefile reader.
$(eval endif)
all: ; @echo hi
