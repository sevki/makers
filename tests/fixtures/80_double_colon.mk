# Two double-colon entries for the same target re-enter the file through
# enter_file's double-colon insert branch (the second `foo::` links onto the
# existing file's double_colon chain). Both rules run, in order.
all: foo
foo:: ; @echo first
foo:: ; @echo second
