# Exercises `load_too_high`'s `/proc/loadavg` probe path. The guard in the job
# scheduler is `job_slots_used() > 0 && load_too_high(ctx)`, so the probe only
# runs when a job is already in flight as another is launched. `a` and `b` are
# independent and each sleep briefly, so under `-j2` they overlap: while one
# holds a slot the scheduler tries to start the other and calls `load_too_high`.
# The cap (`-l 1000`) is never reached, so nothing throttles. Only `all` prints,
# after both finish, so stdout is "done\n" regardless of `a`/`b` timing and
# matches the C oracle byte-for-byte.
all: a b
	@echo done
a: ; @sleep 0.2
b: ; @sleep 0.2
