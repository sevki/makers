all: foo.out
	@echo done

foo.in:
	@echo content > $@

%.out: %.in
	@echo building $@ from $<
	@cp $< $@
