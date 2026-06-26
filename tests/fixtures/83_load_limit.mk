# Exercises `load_too_high`'s `/proc/loadavg` probe path under a parallel build
# (`-j2`) with a load cap (`-l 1000`) that real system load never reaches, so
# the limit never throttles a job start: both binaries probe and read the load
# the same way and run every recipe. The `b: a` dependency forces a then b, so
# stdout is order-stable under `-j` and matches the C oracle byte-for-byte.
all: a b
a: ; @echo a
b: a ; @echo b
