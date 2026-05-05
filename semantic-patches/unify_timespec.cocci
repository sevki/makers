@@
@@

-#[derive(Copy, Clone)]
-#[repr(C)]
-pub struct timespec {
-    pub tv_sec: __time_t,
-    pub tv_nsec: __syscall_slong_t,
-}
+pub use crate::sys_stat::timespec;
