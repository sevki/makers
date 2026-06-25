# Exercises `-j`/`--jobs`, whose resolved slot count is now `Options::job_slots`
# (the former `static mut job_slots`). The targets form a strict dependency
# chain, so the build order is deterministic regardless of `-j` width — `-j1`,
# `-j2`, and the default all produce identical output, matched against the C
# oracle. This checks that the job_slots resolution and the `-j2` jobserver-master
# setup stay behavior-preserving; the kernel-build CI separately exercises real
# parallel scheduling.
all: mid
	@echo built $@

mid: base
	@echo built $@

base:
	@echo built $@
