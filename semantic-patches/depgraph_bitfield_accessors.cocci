// Convert c2rust bitfield accessors on the dependency-graph structs
// (File / Dep / GoalDep / PatDeps) to plain bool field accesses.
// Generated from the flag-name list; regenerate rather than hand-edit.
// NOTE: `child` (job.rs) keeps its bitfields — exclude job.rs accessor hits
// for `dontcare` or convert child separately.

@get_changed_cast_ne0@
expression E;
@@
- E.changed() as ::core::ffi::c_int != 0
+ E.changed

@get_changed_ne0@
expression E;
@@
- E.changed() != 0
+ E.changed

@get_changed_eq0@
expression E;
@@
- E.changed() == 0
+ !E.changed

@set_changed_lit0@
expression E;
@@
- E.set_changed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.changed = true

@set_changed_lit1@
expression E;
@@
- E.set_changed(1 as ::core::ffi::c_uint)
+ E.changed = true

@set_changed_lit2@
expression E;
@@
- E.set_changed(1)
+ E.changed = true

@set_changed_lit3@
expression E;
@@
- E.set_changed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.changed = false

@set_changed_lit4@
expression E;
@@
- E.set_changed(0 as ::core::ffi::c_uint)
+ E.changed = false

@set_changed_lit5@
expression E;
@@
- E.set_changed(0)
+ E.changed = false

@set_changed_expr@
expression E, V;
@@
- E.set_changed(V)
+ E.changed = (V) != 0

@get_changed@
expression E;
@@
- E.changed()
+ E.changed

@get_ignore_mtime_cast_ne0@
expression E;
@@
- E.ignore_mtime() as ::core::ffi::c_int != 0
+ E.ignore_mtime

@get_ignore_mtime_ne0@
expression E;
@@
- E.ignore_mtime() != 0
+ E.ignore_mtime

@get_ignore_mtime_eq0@
expression E;
@@
- E.ignore_mtime() == 0
+ !E.ignore_mtime

@set_ignore_mtime_lit0@
expression E;
@@
- E.set_ignore_mtime(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.ignore_mtime = true

@set_ignore_mtime_lit1@
expression E;
@@
- E.set_ignore_mtime(1 as ::core::ffi::c_uint)
+ E.ignore_mtime = true

@set_ignore_mtime_lit2@
expression E;
@@
- E.set_ignore_mtime(1)
+ E.ignore_mtime = true

@set_ignore_mtime_lit3@
expression E;
@@
- E.set_ignore_mtime(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.ignore_mtime = false

@set_ignore_mtime_lit4@
expression E;
@@
- E.set_ignore_mtime(0 as ::core::ffi::c_uint)
+ E.ignore_mtime = false

@set_ignore_mtime_lit5@
expression E;
@@
- E.set_ignore_mtime(0)
+ E.ignore_mtime = false

@set_ignore_mtime_expr@
expression E, V;
@@
- E.set_ignore_mtime(V)
+ E.ignore_mtime = (V) != 0

@get_ignore_mtime@
expression E;
@@
- E.ignore_mtime()
+ E.ignore_mtime

@get_staticpattern_cast_ne0@
expression E;
@@
- E.staticpattern() as ::core::ffi::c_int != 0
+ E.staticpattern

@get_staticpattern_ne0@
expression E;
@@
- E.staticpattern() != 0
+ E.staticpattern

@get_staticpattern_eq0@
expression E;
@@
- E.staticpattern() == 0
+ !E.staticpattern

@set_staticpattern_lit0@
expression E;
@@
- E.set_staticpattern(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.staticpattern = true

@set_staticpattern_lit1@
expression E;
@@
- E.set_staticpattern(1 as ::core::ffi::c_uint)
+ E.staticpattern = true

@set_staticpattern_lit2@
expression E;
@@
- E.set_staticpattern(1)
+ E.staticpattern = true

@set_staticpattern_lit3@
expression E;
@@
- E.set_staticpattern(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.staticpattern = false

@set_staticpattern_lit4@
expression E;
@@
- E.set_staticpattern(0 as ::core::ffi::c_uint)
+ E.staticpattern = false

@set_staticpattern_lit5@
expression E;
@@
- E.set_staticpattern(0)
+ E.staticpattern = false

@set_staticpattern_expr@
expression E, V;
@@
- E.set_staticpattern(V)
+ E.staticpattern = (V) != 0

@get_staticpattern@
expression E;
@@
- E.staticpattern()
+ E.staticpattern

@get_need_2nd_expansion_cast_ne0@
expression E;
@@
- E.need_2nd_expansion() as ::core::ffi::c_int != 0
+ E.need_2nd_expansion

@get_need_2nd_expansion_ne0@
expression E;
@@
- E.need_2nd_expansion() != 0
+ E.need_2nd_expansion

@get_need_2nd_expansion_eq0@
expression E;
@@
- E.need_2nd_expansion() == 0
+ !E.need_2nd_expansion

@set_need_2nd_expansion_lit0@
expression E;
@@
- E.set_need_2nd_expansion(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.need_2nd_expansion = true

@set_need_2nd_expansion_lit1@
expression E;
@@
- E.set_need_2nd_expansion(1 as ::core::ffi::c_uint)
+ E.need_2nd_expansion = true

@set_need_2nd_expansion_lit2@
expression E;
@@
- E.set_need_2nd_expansion(1)
+ E.need_2nd_expansion = true

@set_need_2nd_expansion_lit3@
expression E;
@@
- E.set_need_2nd_expansion(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.need_2nd_expansion = false

@set_need_2nd_expansion_lit4@
expression E;
@@
- E.set_need_2nd_expansion(0 as ::core::ffi::c_uint)
+ E.need_2nd_expansion = false

@set_need_2nd_expansion_lit5@
expression E;
@@
- E.set_need_2nd_expansion(0)
+ E.need_2nd_expansion = false

@set_need_2nd_expansion_expr@
expression E, V;
@@
- E.set_need_2nd_expansion(V)
+ E.need_2nd_expansion = (V) != 0

@get_need_2nd_expansion@
expression E;
@@
- E.need_2nd_expansion()
+ E.need_2nd_expansion

@get_ignore_automatic_vars_cast_ne0@
expression E;
@@
- E.ignore_automatic_vars() as ::core::ffi::c_int != 0
+ E.ignore_automatic_vars

@get_ignore_automatic_vars_ne0@
expression E;
@@
- E.ignore_automatic_vars() != 0
+ E.ignore_automatic_vars

@get_ignore_automatic_vars_eq0@
expression E;
@@
- E.ignore_automatic_vars() == 0
+ !E.ignore_automatic_vars

@set_ignore_automatic_vars_lit0@
expression E;
@@
- E.set_ignore_automatic_vars(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.ignore_automatic_vars = true

@set_ignore_automatic_vars_lit1@
expression E;
@@
- E.set_ignore_automatic_vars(1 as ::core::ffi::c_uint)
+ E.ignore_automatic_vars = true

@set_ignore_automatic_vars_lit2@
expression E;
@@
- E.set_ignore_automatic_vars(1)
+ E.ignore_automatic_vars = true

@set_ignore_automatic_vars_lit3@
expression E;
@@
- E.set_ignore_automatic_vars(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.ignore_automatic_vars = false

@set_ignore_automatic_vars_lit4@
expression E;
@@
- E.set_ignore_automatic_vars(0 as ::core::ffi::c_uint)
+ E.ignore_automatic_vars = false

@set_ignore_automatic_vars_lit5@
expression E;
@@
- E.set_ignore_automatic_vars(0)
+ E.ignore_automatic_vars = false

@set_ignore_automatic_vars_expr@
expression E, V;
@@
- E.set_ignore_automatic_vars(V)
+ E.ignore_automatic_vars = (V) != 0

@get_ignore_automatic_vars@
expression E;
@@
- E.ignore_automatic_vars()
+ E.ignore_automatic_vars

@get_is_explicit_cast_ne0@
expression E;
@@
- E.is_explicit() as ::core::ffi::c_int != 0
+ E.is_explicit

@get_is_explicit_ne0@
expression E;
@@
- E.is_explicit() != 0
+ E.is_explicit

@get_is_explicit_eq0@
expression E;
@@
- E.is_explicit() == 0
+ !E.is_explicit

@set_is_explicit_lit0@
expression E;
@@
- E.set_is_explicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.is_explicit = true

@set_is_explicit_lit1@
expression E;
@@
- E.set_is_explicit(1 as ::core::ffi::c_uint)
+ E.is_explicit = true

@set_is_explicit_lit2@
expression E;
@@
- E.set_is_explicit(1)
+ E.is_explicit = true

@set_is_explicit_lit3@
expression E;
@@
- E.set_is_explicit(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.is_explicit = false

@set_is_explicit_lit4@
expression E;
@@
- E.set_is_explicit(0 as ::core::ffi::c_uint)
+ E.is_explicit = false

@set_is_explicit_lit5@
expression E;
@@
- E.set_is_explicit(0)
+ E.is_explicit = false

@set_is_explicit_expr@
expression E, V;
@@
- E.set_is_explicit(V)
+ E.is_explicit = (V) != 0

@get_is_explicit@
expression E;
@@
- E.is_explicit()
+ E.is_explicit

@get_wait_here_cast_ne0@
expression E;
@@
- E.wait_here() as ::core::ffi::c_int != 0
+ E.wait_here

@get_wait_here_ne0@
expression E;
@@
- E.wait_here() != 0
+ E.wait_here

@get_wait_here_eq0@
expression E;
@@
- E.wait_here() == 0
+ !E.wait_here

@set_wait_here_lit0@
expression E;
@@
- E.set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.wait_here = true

@set_wait_here_lit1@
expression E;
@@
- E.set_wait_here(1 as ::core::ffi::c_uint)
+ E.wait_here = true

@set_wait_here_lit2@
expression E;
@@
- E.set_wait_here(1)
+ E.wait_here = true

@set_wait_here_lit3@
expression E;
@@
- E.set_wait_here(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.wait_here = false

@set_wait_here_lit4@
expression E;
@@
- E.set_wait_here(0 as ::core::ffi::c_uint)
+ E.wait_here = false

@set_wait_here_lit5@
expression E;
@@
- E.set_wait_here(0)
+ E.wait_here = false

@set_wait_here_expr@
expression E, V;
@@
- E.set_wait_here(V)
+ E.wait_here = (V) != 0

@get_wait_here@
expression E;
@@
- E.wait_here()
+ E.wait_here

@get_builtin_cast_ne0@
expression E;
@@
- E.builtin() as ::core::ffi::c_int != 0
+ E.builtin

@get_builtin_ne0@
expression E;
@@
- E.builtin() != 0
+ E.builtin

@get_builtin_eq0@
expression E;
@@
- E.builtin() == 0
+ !E.builtin

@set_builtin_lit0@
expression E;
@@
- E.set_builtin(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.builtin = true

@set_builtin_lit1@
expression E;
@@
- E.set_builtin(1 as ::core::ffi::c_uint)
+ E.builtin = true

@set_builtin_lit2@
expression E;
@@
- E.set_builtin(1)
+ E.builtin = true

@set_builtin_lit3@
expression E;
@@
- E.set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.builtin = false

@set_builtin_lit4@
expression E;
@@
- E.set_builtin(0 as ::core::ffi::c_uint)
+ E.builtin = false

@set_builtin_lit5@
expression E;
@@
- E.set_builtin(0)
+ E.builtin = false

@set_builtin_expr@
expression E, V;
@@
- E.set_builtin(V)
+ E.builtin = (V) != 0

@get_builtin@
expression E;
@@
- E.builtin()
+ E.builtin

@get_precious_cast_ne0@
expression E;
@@
- E.precious() as ::core::ffi::c_int != 0
+ E.precious

@get_precious_ne0@
expression E;
@@
- E.precious() != 0
+ E.precious

@get_precious_eq0@
expression E;
@@
- E.precious() == 0
+ !E.precious

@set_precious_lit0@
expression E;
@@
- E.set_precious(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.precious = true

@set_precious_lit1@
expression E;
@@
- E.set_precious(1 as ::core::ffi::c_uint)
+ E.precious = true

@set_precious_lit2@
expression E;
@@
- E.set_precious(1)
+ E.precious = true

@set_precious_lit3@
expression E;
@@
- E.set_precious(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.precious = false

@set_precious_lit4@
expression E;
@@
- E.set_precious(0 as ::core::ffi::c_uint)
+ E.precious = false

@set_precious_lit5@
expression E;
@@
- E.set_precious(0)
+ E.precious = false

@set_precious_expr@
expression E, V;
@@
- E.set_precious(V)
+ E.precious = (V) != 0

@get_precious@
expression E;
@@
- E.precious()
+ E.precious

@get_loaded_cast_ne0@
expression E;
@@
- E.loaded() as ::core::ffi::c_int != 0
+ E.loaded

@get_loaded_ne0@
expression E;
@@
- E.loaded() != 0
+ E.loaded

@get_loaded_eq0@
expression E;
@@
- E.loaded() == 0
+ !E.loaded

@set_loaded_lit0@
expression E;
@@
- E.set_loaded(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.loaded = true

@set_loaded_lit1@
expression E;
@@
- E.set_loaded(1 as ::core::ffi::c_uint)
+ E.loaded = true

@set_loaded_lit2@
expression E;
@@
- E.set_loaded(1)
+ E.loaded = true

@set_loaded_lit3@
expression E;
@@
- E.set_loaded(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.loaded = false

@set_loaded_lit4@
expression E;
@@
- E.set_loaded(0 as ::core::ffi::c_uint)
+ E.loaded = false

@set_loaded_lit5@
expression E;
@@
- E.set_loaded(0)
+ E.loaded = false

@set_loaded_expr@
expression E, V;
@@
- E.set_loaded(V)
+ E.loaded = (V) != 0

@get_loaded@
expression E;
@@
- E.loaded()
+ E.loaded

@get_unloaded_cast_ne0@
expression E;
@@
- E.unloaded() as ::core::ffi::c_int != 0
+ E.unloaded

@get_unloaded_ne0@
expression E;
@@
- E.unloaded() != 0
+ E.unloaded

@get_unloaded_eq0@
expression E;
@@
- E.unloaded() == 0
+ !E.unloaded

@set_unloaded_lit0@
expression E;
@@
- E.set_unloaded(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.unloaded = true

@set_unloaded_lit1@
expression E;
@@
- E.set_unloaded(1 as ::core::ffi::c_uint)
+ E.unloaded = true

@set_unloaded_lit2@
expression E;
@@
- E.set_unloaded(1)
+ E.unloaded = true

@set_unloaded_lit3@
expression E;
@@
- E.set_unloaded(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.unloaded = false

@set_unloaded_lit4@
expression E;
@@
- E.set_unloaded(0 as ::core::ffi::c_uint)
+ E.unloaded = false

@set_unloaded_lit5@
expression E;
@@
- E.set_unloaded(0)
+ E.unloaded = false

@set_unloaded_expr@
expression E, V;
@@
- E.set_unloaded(V)
+ E.unloaded = (V) != 0

@get_unloaded@
expression E;
@@
- E.unloaded()
+ E.unloaded

@get_low_resolution_time_cast_ne0@
expression E;
@@
- E.low_resolution_time() as ::core::ffi::c_int != 0
+ E.low_resolution_time

@get_low_resolution_time_ne0@
expression E;
@@
- E.low_resolution_time() != 0
+ E.low_resolution_time

@get_low_resolution_time_eq0@
expression E;
@@
- E.low_resolution_time() == 0
+ !E.low_resolution_time

@set_low_resolution_time_lit0@
expression E;
@@
- E.set_low_resolution_time(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.low_resolution_time = true

@set_low_resolution_time_lit1@
expression E;
@@
- E.set_low_resolution_time(1 as ::core::ffi::c_uint)
+ E.low_resolution_time = true

@set_low_resolution_time_lit2@
expression E;
@@
- E.set_low_resolution_time(1)
+ E.low_resolution_time = true

@set_low_resolution_time_lit3@
expression E;
@@
- E.set_low_resolution_time(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.low_resolution_time = false

@set_low_resolution_time_lit4@
expression E;
@@
- E.set_low_resolution_time(0 as ::core::ffi::c_uint)
+ E.low_resolution_time = false

@set_low_resolution_time_lit5@
expression E;
@@
- E.set_low_resolution_time(0)
+ E.low_resolution_time = false

@set_low_resolution_time_expr@
expression E, V;
@@
- E.set_low_resolution_time(V)
+ E.low_resolution_time = (V) != 0

@get_low_resolution_time@
expression E;
@@
- E.low_resolution_time()
+ E.low_resolution_time

@get_tried_implicit_cast_ne0@
expression E;
@@
- E.tried_implicit() as ::core::ffi::c_int != 0
+ E.tried_implicit

@get_tried_implicit_ne0@
expression E;
@@
- E.tried_implicit() != 0
+ E.tried_implicit

@get_tried_implicit_eq0@
expression E;
@@
- E.tried_implicit() == 0
+ !E.tried_implicit

@set_tried_implicit_lit0@
expression E;
@@
- E.set_tried_implicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.tried_implicit = true

@set_tried_implicit_lit1@
expression E;
@@
- E.set_tried_implicit(1 as ::core::ffi::c_uint)
+ E.tried_implicit = true

@set_tried_implicit_lit2@
expression E;
@@
- E.set_tried_implicit(1)
+ E.tried_implicit = true

@set_tried_implicit_lit3@
expression E;
@@
- E.set_tried_implicit(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.tried_implicit = false

@set_tried_implicit_lit4@
expression E;
@@
- E.set_tried_implicit(0 as ::core::ffi::c_uint)
+ E.tried_implicit = false

@set_tried_implicit_lit5@
expression E;
@@
- E.set_tried_implicit(0)
+ E.tried_implicit = false

@set_tried_implicit_expr@
expression E, V;
@@
- E.set_tried_implicit(V)
+ E.tried_implicit = (V) != 0

@get_tried_implicit@
expression E;
@@
- E.tried_implicit()
+ E.tried_implicit

@get_updating_cast_ne0@
expression E;
@@
- E.updating() as ::core::ffi::c_int != 0
+ E.updating

@get_updating_ne0@
expression E;
@@
- E.updating() != 0
+ E.updating

@get_updating_eq0@
expression E;
@@
- E.updating() == 0
+ !E.updating

@set_updating_lit0@
expression E;
@@
- E.set_updating(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.updating = true

@set_updating_lit1@
expression E;
@@
- E.set_updating(1 as ::core::ffi::c_uint)
+ E.updating = true

@set_updating_lit2@
expression E;
@@
- E.set_updating(1)
+ E.updating = true

@set_updating_lit3@
expression E;
@@
- E.set_updating(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.updating = false

@set_updating_lit4@
expression E;
@@
- E.set_updating(0 as ::core::ffi::c_uint)
+ E.updating = false

@set_updating_lit5@
expression E;
@@
- E.set_updating(0)
+ E.updating = false

@set_updating_expr@
expression E, V;
@@
- E.set_updating(V)
+ E.updating = (V) != 0

@get_updating@
expression E;
@@
- E.updating()
+ E.updating

@get_updated_cast_ne0@
expression E;
@@
- E.updated() as ::core::ffi::c_int != 0
+ E.updated

@get_updated_ne0@
expression E;
@@
- E.updated() != 0
+ E.updated

@get_updated_eq0@
expression E;
@@
- E.updated() == 0
+ !E.updated

@set_updated_lit0@
expression E;
@@
- E.set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.updated = true

@set_updated_lit1@
expression E;
@@
- E.set_updated(1 as ::core::ffi::c_uint)
+ E.updated = true

@set_updated_lit2@
expression E;
@@
- E.set_updated(1)
+ E.updated = true

@set_updated_lit3@
expression E;
@@
- E.set_updated(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.updated = false

@set_updated_lit4@
expression E;
@@
- E.set_updated(0 as ::core::ffi::c_uint)
+ E.updated = false

@set_updated_lit5@
expression E;
@@
- E.set_updated(0)
+ E.updated = false

@set_updated_expr@
expression E, V;
@@
- E.set_updated(V)
+ E.updated = (V) != 0

@get_updated@
expression E;
@@
- E.updated()
+ E.updated

@get_is_target_cast_ne0@
expression E;
@@
- E.is_target() as ::core::ffi::c_int != 0
+ E.is_target

@get_is_target_ne0@
expression E;
@@
- E.is_target() != 0
+ E.is_target

@get_is_target_eq0@
expression E;
@@
- E.is_target() == 0
+ !E.is_target

@set_is_target_lit0@
expression E;
@@
- E.set_is_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.is_target = true

@set_is_target_lit1@
expression E;
@@
- E.set_is_target(1 as ::core::ffi::c_uint)
+ E.is_target = true

@set_is_target_lit2@
expression E;
@@
- E.set_is_target(1)
+ E.is_target = true

@set_is_target_lit3@
expression E;
@@
- E.set_is_target(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.is_target = false

@set_is_target_lit4@
expression E;
@@
- E.set_is_target(0 as ::core::ffi::c_uint)
+ E.is_target = false

@set_is_target_lit5@
expression E;
@@
- E.set_is_target(0)
+ E.is_target = false

@set_is_target_expr@
expression E, V;
@@
- E.set_is_target(V)
+ E.is_target = (V) != 0

@get_is_target@
expression E;
@@
- E.is_target()
+ E.is_target

@get_cmd_target_cast_ne0@
expression E;
@@
- E.cmd_target() as ::core::ffi::c_int != 0
+ E.cmd_target

@get_cmd_target_ne0@
expression E;
@@
- E.cmd_target() != 0
+ E.cmd_target

@get_cmd_target_eq0@
expression E;
@@
- E.cmd_target() == 0
+ !E.cmd_target

@set_cmd_target_lit0@
expression E;
@@
- E.set_cmd_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.cmd_target = true

@set_cmd_target_lit1@
expression E;
@@
- E.set_cmd_target(1 as ::core::ffi::c_uint)
+ E.cmd_target = true

@set_cmd_target_lit2@
expression E;
@@
- E.set_cmd_target(1)
+ E.cmd_target = true

@set_cmd_target_lit3@
expression E;
@@
- E.set_cmd_target(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.cmd_target = false

@set_cmd_target_lit4@
expression E;
@@
- E.set_cmd_target(0 as ::core::ffi::c_uint)
+ E.cmd_target = false

@set_cmd_target_lit5@
expression E;
@@
- E.set_cmd_target(0)
+ E.cmd_target = false

@set_cmd_target_expr@
expression E, V;
@@
- E.set_cmd_target(V)
+ E.cmd_target = (V) != 0

@get_cmd_target@
expression E;
@@
- E.cmd_target()
+ E.cmd_target

@get_phony_cast_ne0@
expression E;
@@
- E.phony() as ::core::ffi::c_int != 0
+ E.phony

@get_phony_ne0@
expression E;
@@
- E.phony() != 0
+ E.phony

@get_phony_eq0@
expression E;
@@
- E.phony() == 0
+ !E.phony

@set_phony_lit0@
expression E;
@@
- E.set_phony(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.phony = true

@set_phony_lit1@
expression E;
@@
- E.set_phony(1 as ::core::ffi::c_uint)
+ E.phony = true

@set_phony_lit2@
expression E;
@@
- E.set_phony(1)
+ E.phony = true

@set_phony_lit3@
expression E;
@@
- E.set_phony(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.phony = false

@set_phony_lit4@
expression E;
@@
- E.set_phony(0 as ::core::ffi::c_uint)
+ E.phony = false

@set_phony_lit5@
expression E;
@@
- E.set_phony(0)
+ E.phony = false

@set_phony_expr@
expression E, V;
@@
- E.set_phony(V)
+ E.phony = (V) != 0

@get_phony@
expression E;
@@
- E.phony()
+ E.phony

@get_intermediate_cast_ne0@
expression E;
@@
- E.intermediate() as ::core::ffi::c_int != 0
+ E.intermediate

@get_intermediate_ne0@
expression E;
@@
- E.intermediate() != 0
+ E.intermediate

@get_intermediate_eq0@
expression E;
@@
- E.intermediate() == 0
+ !E.intermediate

@set_intermediate_lit0@
expression E;
@@
- E.set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.intermediate = true

@set_intermediate_lit1@
expression E;
@@
- E.set_intermediate(1 as ::core::ffi::c_uint)
+ E.intermediate = true

@set_intermediate_lit2@
expression E;
@@
- E.set_intermediate(1)
+ E.intermediate = true

@set_intermediate_lit3@
expression E;
@@
- E.set_intermediate(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.intermediate = false

@set_intermediate_lit4@
expression E;
@@
- E.set_intermediate(0 as ::core::ffi::c_uint)
+ E.intermediate = false

@set_intermediate_lit5@
expression E;
@@
- E.set_intermediate(0)
+ E.intermediate = false

@set_intermediate_expr@
expression E, V;
@@
- E.set_intermediate(V)
+ E.intermediate = (V) != 0

@get_intermediate@
expression E;
@@
- E.intermediate()
+ E.intermediate

@get_secondary_cast_ne0@
expression E;
@@
- E.secondary() as ::core::ffi::c_int != 0
+ E.secondary

@get_secondary_ne0@
expression E;
@@
- E.secondary() != 0
+ E.secondary

@get_secondary_eq0@
expression E;
@@
- E.secondary() == 0
+ !E.secondary

@set_secondary_lit0@
expression E;
@@
- E.set_secondary(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.secondary = true

@set_secondary_lit1@
expression E;
@@
- E.set_secondary(1 as ::core::ffi::c_uint)
+ E.secondary = true

@set_secondary_lit2@
expression E;
@@
- E.set_secondary(1)
+ E.secondary = true

@set_secondary_lit3@
expression E;
@@
- E.set_secondary(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.secondary = false

@set_secondary_lit4@
expression E;
@@
- E.set_secondary(0 as ::core::ffi::c_uint)
+ E.secondary = false

@set_secondary_lit5@
expression E;
@@
- E.set_secondary(0)
+ E.secondary = false

@set_secondary_expr@
expression E, V;
@@
- E.set_secondary(V)
+ E.secondary = (V) != 0

@get_secondary@
expression E;
@@
- E.secondary()
+ E.secondary

@get_notintermediate_cast_ne0@
expression E;
@@
- E.notintermediate() as ::core::ffi::c_int != 0
+ E.notintermediate

@get_notintermediate_ne0@
expression E;
@@
- E.notintermediate() != 0
+ E.notintermediate

@get_notintermediate_eq0@
expression E;
@@
- E.notintermediate() == 0
+ !E.notintermediate

@set_notintermediate_lit0@
expression E;
@@
- E.set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.notintermediate = true

@set_notintermediate_lit1@
expression E;
@@
- E.set_notintermediate(1 as ::core::ffi::c_uint)
+ E.notintermediate = true

@set_notintermediate_lit2@
expression E;
@@
- E.set_notintermediate(1)
+ E.notintermediate = true

@set_notintermediate_lit3@
expression E;
@@
- E.set_notintermediate(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.notintermediate = false

@set_notintermediate_lit4@
expression E;
@@
- E.set_notintermediate(0 as ::core::ffi::c_uint)
+ E.notintermediate = false

@set_notintermediate_lit5@
expression E;
@@
- E.set_notintermediate(0)
+ E.notintermediate = false

@set_notintermediate_expr@
expression E, V;
@@
- E.set_notintermediate(V)
+ E.notintermediate = (V) != 0

@get_notintermediate@
expression E;
@@
- E.notintermediate()
+ E.notintermediate

@get_dontcare_cast_ne0@
expression E;
@@
- E.dontcare() as ::core::ffi::c_int != 0
+ E.dontcare

@get_dontcare_ne0@
expression E;
@@
- E.dontcare() != 0
+ E.dontcare

@get_dontcare_eq0@
expression E;
@@
- E.dontcare() == 0
+ !E.dontcare

@set_dontcare_lit0@
expression E;
@@
- E.set_dontcare(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.dontcare = true

@set_dontcare_lit1@
expression E;
@@
- E.set_dontcare(1 as ::core::ffi::c_uint)
+ E.dontcare = true

@set_dontcare_lit2@
expression E;
@@
- E.set_dontcare(1)
+ E.dontcare = true

@set_dontcare_lit3@
expression E;
@@
- E.set_dontcare(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.dontcare = false

@set_dontcare_lit4@
expression E;
@@
- E.set_dontcare(0 as ::core::ffi::c_uint)
+ E.dontcare = false

@set_dontcare_lit5@
expression E;
@@
- E.set_dontcare(0)
+ E.dontcare = false

@set_dontcare_expr@
expression E, V;
@@
- E.set_dontcare(V)
+ E.dontcare = (V) != 0

@get_dontcare@
expression E;
@@
- E.dontcare()
+ E.dontcare

@get_ignore_vpath_cast_ne0@
expression E;
@@
- E.ignore_vpath() as ::core::ffi::c_int != 0
+ E.ignore_vpath

@get_ignore_vpath_ne0@
expression E;
@@
- E.ignore_vpath() != 0
+ E.ignore_vpath

@get_ignore_vpath_eq0@
expression E;
@@
- E.ignore_vpath() == 0
+ !E.ignore_vpath

@set_ignore_vpath_lit0@
expression E;
@@
- E.set_ignore_vpath(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.ignore_vpath = true

@set_ignore_vpath_lit1@
expression E;
@@
- E.set_ignore_vpath(1 as ::core::ffi::c_uint)
+ E.ignore_vpath = true

@set_ignore_vpath_lit2@
expression E;
@@
- E.set_ignore_vpath(1)
+ E.ignore_vpath = true

@set_ignore_vpath_lit3@
expression E;
@@
- E.set_ignore_vpath(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.ignore_vpath = false

@set_ignore_vpath_lit4@
expression E;
@@
- E.set_ignore_vpath(0 as ::core::ffi::c_uint)
+ E.ignore_vpath = false

@set_ignore_vpath_lit5@
expression E;
@@
- E.set_ignore_vpath(0)
+ E.ignore_vpath = false

@set_ignore_vpath_expr@
expression E, V;
@@
- E.set_ignore_vpath(V)
+ E.ignore_vpath = (V) != 0

@get_ignore_vpath@
expression E;
@@
- E.ignore_vpath()
+ E.ignore_vpath

@get_pat_searched_cast_ne0@
expression E;
@@
- E.pat_searched() as ::core::ffi::c_int != 0
+ E.pat_searched

@get_pat_searched_ne0@
expression E;
@@
- E.pat_searched() != 0
+ E.pat_searched

@get_pat_searched_eq0@
expression E;
@@
- E.pat_searched() == 0
+ !E.pat_searched

@set_pat_searched_lit0@
expression E;
@@
- E.set_pat_searched(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.pat_searched = true

@set_pat_searched_lit1@
expression E;
@@
- E.set_pat_searched(1 as ::core::ffi::c_uint)
+ E.pat_searched = true

@set_pat_searched_lit2@
expression E;
@@
- E.set_pat_searched(1)
+ E.pat_searched = true

@set_pat_searched_lit3@
expression E;
@@
- E.set_pat_searched(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.pat_searched = false

@set_pat_searched_lit4@
expression E;
@@
- E.set_pat_searched(0 as ::core::ffi::c_uint)
+ E.pat_searched = false

@set_pat_searched_lit5@
expression E;
@@
- E.set_pat_searched(0)
+ E.pat_searched = false

@set_pat_searched_expr@
expression E, V;
@@
- E.set_pat_searched(V)
+ E.pat_searched = (V) != 0

@get_pat_searched@
expression E;
@@
- E.pat_searched()
+ E.pat_searched

@get_no_diag_cast_ne0@
expression E;
@@
- E.no_diag() as ::core::ffi::c_int != 0
+ E.no_diag

@get_no_diag_ne0@
expression E;
@@
- E.no_diag() != 0
+ E.no_diag

@get_no_diag_eq0@
expression E;
@@
- E.no_diag() == 0
+ !E.no_diag

@set_no_diag_lit0@
expression E;
@@
- E.set_no_diag(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.no_diag = true

@set_no_diag_lit1@
expression E;
@@
- E.set_no_diag(1 as ::core::ffi::c_uint)
+ E.no_diag = true

@set_no_diag_lit2@
expression E;
@@
- E.set_no_diag(1)
+ E.no_diag = true

@set_no_diag_lit3@
expression E;
@@
- E.set_no_diag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.no_diag = false

@set_no_diag_lit4@
expression E;
@@
- E.set_no_diag(0 as ::core::ffi::c_uint)
+ E.no_diag = false

@set_no_diag_lit5@
expression E;
@@
- E.set_no_diag(0)
+ E.no_diag = false

@set_no_diag_expr@
expression E, V;
@@
- E.set_no_diag(V)
+ E.no_diag = (V) != 0

@get_no_diag@
expression E;
@@
- E.no_diag()
+ E.no_diag

@get_was_shuffled_cast_ne0@
expression E;
@@
- E.was_shuffled() as ::core::ffi::c_int != 0
+ E.was_shuffled

@get_was_shuffled_ne0@
expression E;
@@
- E.was_shuffled() != 0
+ E.was_shuffled

@get_was_shuffled_eq0@
expression E;
@@
- E.was_shuffled() == 0
+ !E.was_shuffled

@set_was_shuffled_lit0@
expression E;
@@
- E.set_was_shuffled(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.was_shuffled = true

@set_was_shuffled_lit1@
expression E;
@@
- E.set_was_shuffled(1 as ::core::ffi::c_uint)
+ E.was_shuffled = true

@set_was_shuffled_lit2@
expression E;
@@
- E.set_was_shuffled(1)
+ E.was_shuffled = true

@set_was_shuffled_lit3@
expression E;
@@
- E.set_was_shuffled(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.was_shuffled = false

@set_was_shuffled_lit4@
expression E;
@@
- E.set_was_shuffled(0 as ::core::ffi::c_uint)
+ E.was_shuffled = false

@set_was_shuffled_lit5@
expression E;
@@
- E.set_was_shuffled(0)
+ E.was_shuffled = false

@set_was_shuffled_expr@
expression E, V;
@@
- E.set_was_shuffled(V)
+ E.was_shuffled = (V) != 0

@get_was_shuffled@
expression E;
@@
- E.was_shuffled()
+ E.was_shuffled

@get_snapped_cast_ne0@
expression E;
@@
- E.snapped() as ::core::ffi::c_int != 0
+ E.snapped

@get_snapped_ne0@
expression E;
@@
- E.snapped() != 0
+ E.snapped

@get_snapped_eq0@
expression E;
@@
- E.snapped() == 0
+ !E.snapped

@set_snapped_lit0@
expression E;
@@
- E.set_snapped(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.snapped = true

@set_snapped_lit1@
expression E;
@@
- E.set_snapped(1 as ::core::ffi::c_uint)
+ E.snapped = true

@set_snapped_lit2@
expression E;
@@
- E.set_snapped(1)
+ E.snapped = true

@set_snapped_lit3@
expression E;
@@
- E.set_snapped(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.snapped = false

@set_snapped_lit4@
expression E;
@@
- E.set_snapped(0 as ::core::ffi::c_uint)
+ E.snapped = false

@set_snapped_lit5@
expression E;
@@
- E.set_snapped(0)
+ E.snapped = false

@set_snapped_expr@
expression E, V;
@@
- E.set_snapped(V)
+ E.snapped = (V) != 0

@get_snapped@
expression E;
@@
- E.snapped()
+ E.snapped

@get_suffix_cast_ne0@
expression E;
@@
- E.suffix() as ::core::ffi::c_int != 0
+ E.suffix

@get_suffix_ne0@
expression E;
@@
- E.suffix() != 0
+ E.suffix

@get_suffix_eq0@
expression E;
@@
- E.suffix() == 0
+ !E.suffix

@set_suffix_lit0@
expression E;
@@
- E.set_suffix(1 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.suffix = true

@set_suffix_lit1@
expression E;
@@
- E.set_suffix(1 as ::core::ffi::c_uint)
+ E.suffix = true

@set_suffix_lit2@
expression E;
@@
- E.set_suffix(1)
+ E.suffix = true

@set_suffix_lit3@
expression E;
@@
- E.set_suffix(0 as ::core::ffi::c_uint as ::core::ffi::c_uint)
+ E.suffix = false

@set_suffix_lit4@
expression E;
@@
- E.set_suffix(0 as ::core::ffi::c_uint)
+ E.suffix = false

@set_suffix_lit5@
expression E;
@@
- E.set_suffix(0)
+ E.suffix = false

@set_suffix_expr@
expression E, V;
@@
- E.set_suffix(V)
+ E.suffix = (V) != 0

@get_suffix@
expression E;
@@
- E.suffix()
+ E.suffix
