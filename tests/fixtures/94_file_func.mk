# $(file ...) — write, append, read, and the missing-file path. The fixture
# harness snapshots the working tree, so out.txt/empty.txt byte-compare
# against the C oracle too.
$(file >out.txt,first line)
$(file >>out.txt,second line)
$(file >>out.txt,)
$(file >empty.txt)
X := $(file <out.txt)
$(info read=[$(X)])
Y := $(file <missing.txt)
$(info missing=[$(Y)])

all:
	@printf 'line with crlf\r\n' > crlf.txt
	@echo done

check: all
	@echo crlf=[$(file <crlf.txt)]
