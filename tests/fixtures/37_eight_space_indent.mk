# Exercises eval's "missing separator (did you mean TAB instead of 8 spaces?)"
# diagnostic (now classified in the typed AST layer): the recipe line below is
# indented with eight spaces instead of a TAB, so make must emit the specific
# hint. Both binaries must match byte-for-byte.
all:
        @echo hi
