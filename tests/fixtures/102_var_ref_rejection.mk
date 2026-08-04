# Exercises the reference paths that #442 turned into `Result` rejections:
# a substitution reference to an undefined variable (expand.rs
# expand_string_buf) and a plain reference to one (expand_variable_output).
# Both are forced at parse time with `:=` so the diagnostic's position does
# not depend on recipe scheduling. Under --warn=undefined-var:error the first
# reference is raised as a BuildError that unwinds through those frames
# instead of ending the process in place.
BASE = one
override BASE += two
SUB := $(MISSING:a=b)
PLAIN := $(ALSO_MISSING)

all:
	@echo appended $(BASE) sub "$(SUB)" plain "$(PLAIN)"
