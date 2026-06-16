//! Recipe (command) handling: chopping recipes into lines, setting the
//! automatic variables (`$@`, `$<`, `$^`, ...), running a target's
//! commands, and cleaning up half-built targets on a fatal signal.
//!
//! Port of `commands.c`.

use crate::ar::{ar_member_date, ar_name};
pub use crate::ffi_types::{pid_t, sig_atomic_t, size_t, time_t, uintmax_t};
use crate::file::{
    cs_running, enter_file, file_timestamp_cons, remove_intermediates, set_command_state,
    update_status, us_success, Commands, Dep, File, NONEXISTENT_MTIME, ORDINARY_MTIME_MIN,
};
use crate::floc::Floc;
use crate::hash::{
    hash_find_item, hash_find_slot, hash_free, hash_init, hash_insert_at, hash_table, is_real_item,
    jhash_string,
};
use crate::job::{child, children, job_slots_used, new_job, reap_children};
use crate::load::unload_file;
use crate::make_main::{
    always_make_flag, cmd_prefix, default_file, one_shell, stopchar_map, temp_stdin_unlink,
};
use crate::misc::{make_pid, xmalloc, xrealloc, xstrdup, xstrndup};
use crate::output::{error, fatal, perror_with_name, pfatal_with_name, INTSTR_LENGTH};
use crate::posixos::{jobserver_clear, osync_clear};
use crate::remake::notice_finished_file;
use crate::stdio::FILE;
use crate::strcache::{strcache_add, strcache_add_len};
use crate::variable::{define_variable_in_set, initialize_file_variables, o_automatic};

use ::core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void, CStr};
use ::core::ptr::{null, null_mut};

use libc::{
    __errno_location, exit, kill, memcmp, printf, puts, signal, strchr, strcmp, strlen, strstr,
    unlink, EINTR, ENOENT, SIGHUP, SIGINT, SIGQUIT, SIGTERM, SIG_DFL, S_IFMT, S_IFREG,
};

extern "C" {
    static mut stdout: *mut FILE;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn mempcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

pub type file = File;
pub type dep = Dep;
pub type commands = Commands;

pub const MAKE_TROUBLE: c_int = 1;

/// Recipe-line flag bits stored in `commands.lines_flags`.
pub const COMMANDS_RECURSE: c_int = 1;
pub const COMMANDS_SILENT: c_int = 2;
pub const COMMANDS_NOERROR: c_int = 4;

pub const FILE_LIST_SEPARATOR: c_char = b' ' as c_char;

/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_BLANK: c_int = 0x0002;
const MAP_NEWLINE: c_int = 0x0004;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
fn stop_set(c: c_char, mask: c_int) -> bool {
    stopchar_map()[c as u8 as usize] as c_int & mask != 0
}

/// The name a dependency goes by: its own name, or its file's name when
/// the dep node has none (`dep_name` from `dep.h`).
unsafe fn dep_name(d: *const Dep) -> *const c_char {
    if !(*d).name.is_null() {
        (*d).name
    } else {
        (*d).file.as_ref().expect("a nameless dep has a file").name
    }
}

/// Hash a dependency by its name (primary hash for the `$^` dedupe table).
///
/// # Safety
///
/// `key` must point to a valid `dep` whose name (or file name) is a
/// NUL-terminated string.
pub unsafe fn dep_hash_1(key: *const c_void) -> c_ulong {
    jhash_string(::core::ffi::CStr::from_ptr(dep_name(key as *const dep)).to_bytes()) as c_ulong
}

/// Secondary hash for [`dep`] keys; always zero, kept for the callback ABI.
/// The raw key pointer is accepted to match the signature but never inspected.
pub fn dep_hash_2(_key: *const c_void) -> c_ulong {
    0
}

unsafe fn dep_hash_cmp(x: *const c_void, y: *const c_void) -> c_int {
    strcmp(dep_name(x as *const dep), dep_name(y as *const dep))
}

/// Overwrite the trailing separator of a space-joined list (or write an
/// empty string when nothing was appended).
unsafe fn finish_list(start: *mut c_char, end: *mut c_char) {
    if end > start {
        *end.sub(1) = 0;
    } else {
        *end = 0;
    }
}

unsafe fn define_automatic(file: &mut File, name: &CStr, value: *const c_char) {
    define_variable_in_set(
        name.as_ptr(),
        name.to_bytes().len() as size_t,
        value,
        o_automatic,
        0,
        file.variables
            .as_ref()
            .expect("file variables must be initialized before setting automatics")
            .set,
        null::<Floc>(),
    );
}

/// Split an archive reference `lib(member)` into its library and member byte
/// slices. The member excludes the trailing `)`. Pure: indexes the byte view,
/// with no pointer arithmetic. Returns `None` when there is no `(`.
fn split_archive_ref(name: &[u8]) -> Option<(&[u8], &[u8])> {
    let paren = name.iter().position(|&b| b == b'(')?;
    let lib = &name[..paren];
    // The member runs from just after '(' up to the trailing ')'.
    let member = &name[paren + 1..name.len().saturating_sub(1)];
    Some((lib, member))
}

/// Set the automatic variables (`$@`, `$<`, `$*`, `$%`, `$^`, `$+`, `$?`,
/// `$|`) in `file`'s variable set, computing the stem first if needed.
///
/// # Safety
///
/// `file` must be a valid file with initialized per-file variables; `stem`
/// must be null or a NUL-terminated string that outlives the call.
pub unsafe fn set_file_variables(file: *mut file, mut stem: *const c_char) {
    let file = file.as_mut().expect("set_file_variables: null file");

    // For an archive member `lib(member)`, `$@` is `lib` and `$%` is
    // `member`; these buffers replace the C alloca copies.
    let mut at_buf: Vec<u8>;
    let mut percent_buf: Vec<u8>;
    let at: *const c_char;
    let percent: *const c_char;
    if ar_name(::core::ffi::CStr::from_ptr(file.name)) {
        let (lib, member) = split_archive_ref(CStr::from_ptr(file.name).to_bytes())
            .expect("ar_name guarantees a lib(member) reference");
        at_buf = lib.to_vec();
        at_buf.push(0);
        at = at_buf.as_ptr() as *const c_char;

        percent_buf = member.to_vec();
        percent_buf.push(0);
        percent = percent_buf.as_ptr() as *const c_char;
    } else {
        at = file.name;
        percent = c"".as_ptr();
    }

    // If we don't have a stem, derive one by stripping a known suffix.
    if stem.is_null() {
        let name: *const c_char;
        let len: size_t;
        if ar_name(::core::ffi::CStr::from_ptr(file.name)) {
            name = strchr(file.name, '(' as c_int).add(1);
            len = strlen(name) - 1;
        } else {
            name = file.name;
            len = strlen(name);
        }

        let mut d = (*enter_file(strcache_add(c".SUFFIXES".as_ptr()))).deps;
        while !d.is_null() {
            let dn = dep_name(d);
            let slen = strlen(dn);
            if len > slen
                && memcmp(
                    dn as *const c_void,
                    name.add((len - slen) as usize) as *const c_void,
                    slen,
                ) == 0
            {
                stem = strcache_add_len(name, len - slen);
                file.stem = stem;
                break;
            }
            d = (*d).next;
        }
        if d.is_null() {
            stem = c"".as_ptr();
            file.stem = stem;
        }
    }
    let star = stem;

    // `$<` is the first dependency, or `$@` when running default commands.
    let mut less = c"".as_ptr();
    let mut d = file.deps;
    while !d.is_null() {
        if (*d).ignore_mtime() == 0
            && (*d).ignore_automatic_vars() == 0
            && (*d).need_2nd_expansion() == 0
        {
            less = dep_name(d);
            break;
        }
        d = (*d).next;
    }
    if !file.cmds.is_null() && file.cmds == (*default_file).cmds {
        less = at;
    }

    define_automatic(file, c"<", less);
    define_automatic(file, c"*", star);
    define_automatic(file, c"@", at);
    define_automatic(file, c"%", percent);

    // Grow-on-demand buffers for `$+` / `$|` / `$?`, kept across calls
    // exactly like the C statics.
    static mut plus_value: *mut c_char = null_mut();
    static mut bar_value: *mut c_char = null_mut();
    static mut qmark_value: *mut c_char = null_mut();
    static mut plus_max: size_t = 0;
    static mut bar_max: size_t = 0;
    static mut qmark_max: size_t = 0;

    // Total the sizes: `$+` gets every non-order-only dep (with repeats),
    // `$|` every order-only dep.
    let mut plus_len: size_t = 0;
    let mut bar_len: size_t = 0;
    let mut d = file.deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 && (*d).ignore_automatic_vars() == 0 {
            let len = strlen(dep_name(d)) + 1;
            if (*d).ignore_mtime() != 0 {
                bar_len += len;
            } else {
                plus_len += len;
            }
        }
        d = (*d).next;
    }
    if bar_len == 0 {
        bar_len = 1;
    }
    if plus_len == 0 {
        plus_len = 1;
    }

    if plus_len > plus_max {
        plus_max = plus_len;
        plus_value = xrealloc(plus_value as *mut c_void, plus_max) as *mut c_char;
    }

    // Fill `$+`, remembering how much of it can possibly appear in `$?`.
    let mut cp = plus_value;
    let mut qmark_len = plus_len + 1;
    let mut d = file.deps;
    while !d.is_null() {
        if (*d).ignore_mtime() == 0
            && (*d).need_2nd_expansion() == 0
            && (*d).ignore_automatic_vars() == 0
        {
            let mut c = dep_name(d);
            let len;
            if ar_name(::core::ffi::CStr::from_ptr(c)) {
                c = strchr(c, '(' as c_int).add(1);
                len = strlen(c) - 1;
            } else {
                len = strlen(c);
            }
            cp = mempcpy(cp as *mut c_void, c as *const c_void, len) as *mut c_char;
            *cp = FILE_LIST_SEPARATOR;
            cp = cp.add(1);
            if !((*d).changed() != 0 || always_make_flag != 0) {
                qmark_len -= len + 1;
            }
        }
        d = (*d).next;
    }
    finish_list(plus_value, cp);
    define_automatic(file, c"+", plus_value);

    if qmark_len > qmark_max {
        qmark_max = qmark_len;
        qmark_value = xrealloc(qmark_value as *mut c_void, qmark_max) as *mut c_char;
    }
    if bar_len > bar_max {
        bar_max = bar_len;
        bar_value = xrealloc(bar_value as *mut c_void, bar_max) as *mut c_char;
    }

    // `$^` and `$?` must not repeat names, so dedupe deps through a hash
    // table keyed by name.
    let mut dep_hash: hash_table = ::core::mem::zeroed();
    hash_init(
        &raw mut dep_hash,
        500,
        Some(dep_hash_1),
        Some(dep_hash_2),
        Some(dep_hash_cmp),
    );

    let mut d = file.deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 && (*d).ignore_automatic_vars() == 0 {
            let slot = hash_find_slot(&raw mut dep_hash, d as *const c_void)
                .as_mut()
                .expect("hash_find_slot always returns a slot");
            if !is_real_item(*slot) {
                hash_insert_at(
                    &raw mut dep_hash,
                    d as *const c_void,
                    (&raw mut *slot).cast(),
                );
            } else {
                // Already seen: an order-only duplicate of a normal dep
                // is promoted to normal, on both nodes.
                let hd = (*slot as *mut dep)
                    .as_mut()
                    .expect("dedupe table stored a null dep");
                if (*d).ignore_mtime() != hd.ignore_mtime() {
                    hd.set_ignore_mtime(0);
                    (*d).set_ignore_mtime(0);
                }
            }
        }
        d = (*d).next;
    }

    let caret_value = plus_value;
    let mut cp = caret_value;
    let mut qp = qmark_value;
    let mut bp = bar_value;
    let mut d = file.deps;
    while !d.is_null() {
        // Take only each name's canonical (first-inserted) dep node.
        if (*d).need_2nd_expansion() == 0
            && (*d).ignore_automatic_vars() == 0
            && hash_find_item(&raw mut dep_hash, d as *const c_void) == d as *mut c_void
        {
            let mut c = dep_name(d);
            let len;
            if ar_name(::core::ffi::CStr::from_ptr(c)) {
                c = strchr(c, '(' as c_int).add(1);
                len = strlen(c) - 1;
            } else {
                len = strlen(c);
            }
            if (*d).ignore_mtime() != 0 {
                bp = mempcpy(bp as *mut c_void, c as *const c_void, len) as *mut c_char;
                *bp = FILE_LIST_SEPARATOR;
                bp = bp.add(1);
            } else {
                cp = mempcpy(cp as *mut c_void, c as *const c_void, len) as *mut c_char;
                *cp = FILE_LIST_SEPARATOR;
                cp = cp.add(1);
                if (*d).changed() != 0 || always_make_flag != 0 {
                    qp = mempcpy(qp as *mut c_void, c as *const c_void, len) as *mut c_char;
                    *qp = FILE_LIST_SEPARATOR;
                    qp = qp.add(1);
                }
            }
        }
        d = (*d).next;
    }
    hash_free(&raw mut dep_hash, 0);

    finish_list(caret_value, cp);
    define_automatic(file, c"^", caret_value);
    finish_list(qmark_value, qp);
    define_automatic(file, c"?", qmark_value);
    finish_list(bar_value, bp);
    define_automatic(file, c"|", bar_value);
}

/// Split `cmds.commands` into individual recipe lines (respecting
/// backslash-newline continuations) and record each line's `+`/`@`/`-`
/// prefix flags.
///
/// # Safety
///
/// `cmds` must be null or a valid `commands` whose `commands` string is
/// NUL-terminated.
pub unsafe fn chop_commands(cmds: *mut commands) {
    // Recipes are chopped lazily; only do it once.
    let Some(cmds) = cmds.as_mut() else { return };
    if !cmds.command_lines.is_null() {
        return;
    }

    let mut nlines: c_ushort;
    let mut lines: *mut *mut c_char;
    if one_shell != 0 {
        // .ONESHELL: the entire recipe is a single line (sans final newline).
        let l = strlen(cmds.commands) as usize;
        nlines = 1;
        lines = xmalloc(::core::mem::size_of::<*mut c_char>() as size_t) as *mut *mut c_char;
        *lines = xstrdup(cmds.commands);
        if l > 0 && *(*lines).add(l - 1) == b'\n' as c_char {
            *(*lines).add(l - 1) = 0;
        }
    } else {
        let mut p: *const c_char = cmds.commands;
        let mut max: size_t = 5;
        nlines = 0;
        lines = xmalloc(max * ::core::mem::size_of::<*mut c_char>() as size_t) as *mut *mut c_char;
        while *p != 0 {
            // Find the end of this line: an unescaped newline (count the
            // backslashes preceding it) or the end of the recipe.
            let mut end: *const c_char = p;
            loop {
                end = strchr(end, '\n' as c_int);
                if end.is_null() {
                    end = p.add(strlen(p) as usize);
                    break;
                }
                if !(end > p && *end.sub(1) == b'\\' as c_char) {
                    break;
                }
                let mut backslash = true;
                if end > p.add(1) {
                    let mut b = end.sub(2);
                    while b >= p && *b == b'\\' as c_char {
                        backslash = !backslash;
                        b = b.sub(1);
                    }
                }
                if !backslash {
                    break;
                }
                end = end.add(1);
            }

            if nlines as c_int == c_ushort::MAX as c_int {
                fatal(
                    &raw mut cmds.fileinfo,
                    INTSTR_LENGTH,
                    c"recipe has too many lines (limit %hu)".as_ptr(),
                    nlines as c_int,
                );
            }
            if nlines as size_t == max {
                max += 2;
                lines = xrealloc(
                    lines as *mut c_void,
                    max * ::core::mem::size_of::<*mut c_char>() as size_t,
                ) as *mut *mut c_char;
            }
            *lines.add(nlines as usize) = xstrndup(p, end.offset_from(p) as size_t);
            nlines += 1;
            p = end;
            if *p != 0 {
                p = p.add(1);
            }
        }
    }

    cmds.ncommand_lines = nlines;
    cmds.command_lines = lines;
    cmds.set_any_recurse(0);
    cmds.lines_flags = xmalloc(nlines as size_t) as *mut c_uchar;
    for i in 0..nlines as usize {
        let mut flags: c_uchar = 0;
        let mut p: *const c_char = *lines.add(i);
        while stop_set(*p, MAP_BLANK)
            || *p == b'-' as c_char
            || *p == b'@' as c_char
            || *p == b'+' as c_char
        {
            match *p as u8 {
                b'+' => flags |= COMMANDS_RECURSE as c_uchar,
                b'@' => flags |= COMMANDS_SILENT as c_uchar,
                b'-' => flags |= COMMANDS_NOERROR as c_uchar,
                _ => {}
            }
            p = p.add(1);
        }
        // A line invoking $(MAKE) recurses even without a `+` prefix.
        if flags as c_int & COMMANDS_RECURSE == 0
            && (!strstr(p, c"$(MAKE)".as_ptr()).is_null()
                || !strstr(p, c"${MAKE}".as_ptr()).is_null())
        {
            flags |= COMMANDS_RECURSE as c_uchar;
        }
        *cmds.lines_flags.add(i) = flags;
        cmds.set_any_recurse(
            cmds.any_recurse() | (flags as c_int & COMMANDS_RECURSE != 0) as c_uint,
        );
    }
}

/// Run `file`'s commands: set up its variables and start a job, or mark it
/// finished immediately when the recipe is effectively empty.
///
/// # Safety
///
/// `file` must be a valid file with a non-null `cmds`.
pub unsafe fn execute_file_commands(file: *mut file) {
    // A recipe of nothing but whitespace and `-`/`@`/`+` prefixes means
    // there is nothing to execute.
    let mut p: *const c_char = (*file)
        .cmds
        .as_ref()
        .expect("execute_file_commands requires non-null cmds")
        .commands;
    while *p != 0 {
        if !stop_set(*p, MAP_BLANK | MAP_NEWLINE)
            && *p != b'-' as c_char
            && *p != b'@' as c_char
            && *p != b'+' as c_char
        {
            break;
        }
        p = p.add(1);
    }
    if *p == 0 {
        set_command_state(file, cs_running);
        (*file).set_update_status(us_success as update_status);
        notice_finished_file(file);
        return;
    }

    initialize_file_variables(file, 0);
    set_file_variables(file, (*file).stem);

    // A loaded dynamic object being rebuilt must be unloaded first.
    if (*file).loaded() != 0 && unload_file((*file).name) == 0 {
        (*file).set_loaded(0);
        (*file).set_unloaded(1);
    }

    new_job(file);
}

/// Nonzero while a fatal signal is being handled; checked by code that
/// must not re-enter (e.g. output sync teardown).
pub static mut handling_fatal_signal: sig_atomic_t = 0;

/// Handle a fatal signal: kill children, delete half-built targets, then
/// re-raise the signal with the default disposition.
///
/// # Safety
///
/// Only callable as a signal handler (or from one); touches global job
/// state.
pub unsafe extern "C" fn fatal_error_signal(sig: c_int) {
    ::core::ptr::write_volatile(&raw mut handling_fatal_signal, 1);
    signal(sig, SIG_DFL);
    temp_stdin_unlink();
    osync_clear();
    jobserver_clear();

    if sig == SIGTERM {
        // Pass SIGTERM on to children right away so they die with us.
        let mut c = children;
        while !c.is_null() {
            if (*c).remote() == 0 && (*c).pid > 0 {
                kill((*c).pid, SIGTERM);
            }
            c = (*c).next;
        }
    }

    if sig == SIGTERM || sig == SIGINT || sig == SIGHUP || sig == SIGQUIT {
        let mut c = children;
        while !c.is_null() {
            if (*c).remote() != 0 && (*c).pid > 0 {
                crate::remote_stub::remote_kill((*c).pid, sig);
            }
            c = (*c).next;
        }
        let mut c = children;
        while !c.is_null() {
            delete_child_targets(c);
            c = (*c).next;
        }
        // Wait for them all to die before cleaning up.
        while job_slots_used() > 0 {
            reap_children(1, 0);
        }
    } else {
        while job_slots_used() > 0 {
            reap_children(1, 1);
        }
    }

    remove_intermediates(1);

    if sig == SIGQUIT {
        exit(MAKE_TROUBLE);
    }

    // Re-raise with the default handler so our exit status reflects the
    // signal.
    if kill(make_pid(), sig) < 0 {
        pfatal_with_name(c"kill".as_ptr());
    }
}

/// Delete `file` if it exists and was modified since make last recorded
/// its timestamp (i.e. it is a half-finished build product).
unsafe fn delete_target(file: *mut file, on_behalf_of: *const c_char) {
    let file = file.as_mut().expect("delete_target: null file");
    if file.precious() != 0 || file.phony() != 0 {
        return;
    }

    // An archive member can't be unlinked; just warn if it looks touched.
    if ar_name(::core::ffi::CStr::from_ptr(file.name)) {
        let file_date: time_t = if file.last_mtime == NONEXISTENT_MTIME as uintmax_t {
            -1
        } else {
            (file
                .last_mtime
                .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }) as time_t
        };
        if ar_member_date(file.name) != file_date {
            if !on_behalf_of.is_null() {
                error(
                    null::<Floc>(),
                    strlen(on_behalf_of) + strlen(file.name),
                    c"*** [%s] archive member '%s' may be bogus; not deleted".as_ptr(),
                    on_behalf_of,
                    file.name,
                );
            } else {
                error(
                    null::<Floc>(),
                    strlen(file.name),
                    c"*** archive member '%s' may be bogus; not deleted".as_ptr(),
                    file.name,
                );
            }
        }
        return;
    }

    let mut st: libc::stat = ::core::mem::zeroed();
    let mut e: c_int;
    loop {
        e = libc::stat(file.name, &mut st);
        if !(e == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if e == 0
        && st.st_mode & S_IFMT == S_IFREG
        && file_timestamp_cons(file.name, st.st_mtime as time_t, st.st_mtime_nsec as c_long)
            != file.last_mtime
    {
        if !on_behalf_of.is_null() {
            error(
                null::<Floc>(),
                strlen(on_behalf_of) + strlen(file.name),
                c"*** [%s] deleting file '%s'".as_ptr(),
                on_behalf_of,
                file.name,
            );
        } else {
            error(
                null::<Floc>(),
                strlen(file.name),
                c"*** deleting file '%s'".as_ptr(),
                file.name,
            );
        }
        if unlink(file.name) < 0 && *__errno_location() != ENOENT {
            perror_with_name(c"unlink: ".as_ptr(), file.name);
        }
    }
}

/// Delete the targets of `child` (and everything its rule also makes) if
/// they might be incompletely built.
///
/// # Safety
///
/// `child` must be a valid child record.
pub unsafe fn delete_child_targets(child: *mut child) {
    if (*child).deleted() != 0 || (*child).pid < 0 {
        return;
    }
    delete_target((*child).file, null());
    let cf = (*child).file.as_ref().expect("a started child has a file");
    let mut d = cf.also_make;
    while !d.is_null() {
        delete_target((*d).file, cf.name);
        d = (*d).next;
    }
    (*child).set_deleted(1);
}

/// Print `cmds` for `make -p`, one line per recipe line with the command
/// prefix.
///
/// # Safety
///
/// `cmds` must be a valid `commands` with a NUL-terminated recipe.
pub unsafe fn print_commands(cmds: *const commands) {
    fputs(c"#  recipe to execute".as_ptr(), stdout);
    if (*cmds).fileinfo.filenm.is_null() {
        puts(c" (built-in):".as_ptr());
    } else {
        printf(
            c" (from '%s', line %lu):\n".as_ptr(),
            (*cmds).fileinfo.filenm,
            (*cmds).fileinfo.lineno,
        );
    }

    let mut s: *const c_char = (*cmds).commands;
    while *s != 0 {
        // A recipe line ends at an unescaped newline.
        let mut end = s;
        let mut bs = false;
        while *end != 0 {
            if *end == b'\n' as c_char && !bs {
                break;
            }
            bs = if *end == b'\\' as c_char { !bs } else { false };
            end = end.add(1);
        }
        printf(
            c"%c%.*s\n".as_ptr(),
            cmd_prefix as c_int,
            end.offset_from(s) as c_int,
            s,
        );
        s = end.add((*end == b'\n' as c_char) as usize);
    }
}

pub const FILE_TIMESTAMP_HI_RES: c_int = 1;

#[cfg(test)]
mod split_archive_ref_tests {
    use super::split_archive_ref;

    #[test]
    fn splits_lib_and_member() {
        assert_eq!(
            split_archive_ref(b"libfoo.a(bar.o)"),
            Some((b"libfoo.a".as_slice(), b"bar.o".as_slice()))
        );
    }

    #[test]
    fn empty_library_part() {
        // "(member)" — nothing before the paren.
        assert_eq!(
            split_archive_ref(b"(bar.o)"),
            Some((b"".as_slice(), b"bar.o".as_slice()))
        );
    }

    #[test]
    fn empty_member_part() {
        // "lib()" — the member between '(' and ')' is empty.
        assert_eq!(
            split_archive_ref(b"lib()"),
            Some((b"lib".as_slice(), b"".as_slice()))
        );
    }

    #[test]
    fn no_paren_returns_none() {
        assert_eq!(split_archive_ref(b"plainfile.o"), None);
    }
}

#[cfg(test)]
mod hash_2_tests {
    //! The secondary-hash callbacks are constant-zero and never inspect
    //! their key pointer, so they are now safe `fn`s. Exercise each across
    //! the modules touched by this pass with both a null and a non-null key
    //! to confirm the pointer is ignored and the result is 0.
    use core::ffi::c_void;
    use core::ptr;

    #[test]
    fn secondary_hashes_are_zero_and_ignore_key() {
        let dummy = 0xdead_beef_usize as *const c_void;
        for key in [ptr::null::<c_void>(), dummy] {
            assert_eq!(crate::commands::dep_hash_2(key), 0);
            assert_eq!(crate::dir::directory_hash_2(key), 0);
            assert_eq!(crate::dir::dirfile_hash_2(key), 0);
            assert_eq!(crate::file::file_hash_2(key), 0);
            assert_eq!(crate::variable::variable_hash_2(key), 0);
            assert_eq!(crate::function::a_word_hash_2(key), 0);
        }
    }
}
