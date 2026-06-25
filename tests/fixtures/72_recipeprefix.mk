# Exercises `.RECIPEPREFIX`, which changes the recipe-introducing character
# from a tab to `>` here, now resolved through `Options::cmd_prefix` instead of
# the former `static mut cmd_prefix`. After the assignment, the makefile reader
# must classify `>`-prefixed lines (not tab-prefixed ones) as recipes. Output is
# differential-checked against the C oracle. Note: the recipe lines below begin
# with `>`, not a tab.
.RECIPEPREFIX = >
all: dep
>echo building $@
>echo done $@

dep:
>echo building $@
