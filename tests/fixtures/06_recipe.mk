# Tests recipe-line parsing: line continuations, automatic vars, $$ escaping.
TARGETS = one two three

all: $(TARGETS)

$(TARGETS):
	@echo "$@: building from $^ (auto var \$$\$$ -> $$)"
	@for x in 1 2 3; do echo "  step-$$x for $@"; done

# Multi-line recipe with backslash continuation in a variable
LINES := first \
         second \
         third
.PHONY: lines
lines:
	@echo lines=$(LINES)

.PHONY: all $(TARGETS)
