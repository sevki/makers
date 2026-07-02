use crate::output::FmtArg;
use libc::{fnmatch, free, strchr};

pub use crate::ffi_types::{__time_t, intmax_t, size_t, time_t, uintmax_t};
use crate::file::{Dep, File, SeqNode};
use crate::misc::xstrdup;
use crate::strcache::strcache_add;
extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type file = File;
pub type cmd_state = ::core::ffi::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = ::core::ffi::c_uint;
pub type update_status_0 = u32;
pub const us_failed: update_status_0 = 3;
pub const us_question: update_status_0 = 2;
pub const us_none: update_status_0 = 1;
pub const us_success: update_status_0 = 0;
pub type dep = Dep;
use crate::floc::Floc;

pub type ar_member_func_t = Option<
    unsafe fn(
        i32,
        *const ::core::ffi::c_char,
        i32,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        intmax_t,
        i32,
        i32,
        ::core::ffi::c_uint,
        *const ::core::ffi::c_void,
    ) -> intmax_t,
>;
use crate::arscan::{ar_member_touch, ar_name_equal, ar_scan};
use crate::dir::file_exists_p;
pub use crate::file::nameseq;
use crate::file::{enter_file, lookup_file};
use crate::misc::{alpha_cmp, concat};
use crate::output::{error, fatal, out_of_memory, perror_with_name};
use crate::remake::f_mtime;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArGlobState<T: SeqNode> {
    /// The session context, for interning matched member names. Raw pointer
    /// because this state crosses the C-shaped `ar_scan` callback protocol;
    /// it points at `ar_glob`'s caller-borrowed context and never outlives
    /// the `ar_scan` call.
    pub ctx: *const crate::execctx::ExecContext,
    pub arname: *const ::core::ffi::c_char,
    pub pattern: *const ::core::ffi::c_char,
    pub chain: *mut T,
    pub n: ::core::ffi::c_uint,
}
pub const CHAR_BIT: i32 = __CHAR_BIT__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FNM_PATHNAME: i32 = (1) << 0;
pub const FNM_PERIOD: i32 = (1) << 2;
/// Classification of a target name with respect to the `archive(member)`
/// syntax recognized by [`ar_name`].
enum ArName {
    /// Not an archive reference: no usable `(member)` suffix.
    Plain,
    /// A well-formed `archive(member)` reference.
    Member,
    /// The unsupported nested `archive((member))` form.
    Unsupported,
}

/// Classify `bytes` (a target name, without its terminating NUL) as an archive
/// reference. Pure mirror of make's `ar_name` parsing logic.
fn classify_ar_name(bytes: &[u8]) -> ArName {
    // Find the first '('; it must exist and not be the very first byte.
    let lp = match bytes.iter().position(|&c| c == b'(') {
        None | Some(0) => return ArName::Plain,
        Some(i) => i,
    };
    // The name must end with ')', and the member must be non-empty (the ')'
    // cannot sit immediately after the '(').
    let last = bytes.len() - 1;
    if bytes[last] != b')' || last == lp + 1 {
        return ArName::Plain;
    }
    // `archive((member))` is the unsupported nested form.
    if bytes[lp + 1] == b'(' && bytes[last - 1] == b')' {
        return ArName::Unsupported;
    }
    ArName::Member
}

/// Does `name` refer to an `archive(member)` target?
///
/// Aborts via [`fatal`] on the unsupported nested `archive((member))` form,
/// matching make's behavior.
pub fn ar_name(ctx: &crate::execctx::ExecContext, name: &::core::ffi::CStr) -> bool {
    match classify_ar_name(name.to_bytes()) {
        ArName::Plain => false,
        ArName::Member => true,
        ArName::Unsupported => unsafe {
            fatal(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        name.to_bytes().len() as size_t,
        b"attempt to use unsupported feature: '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
        &[FmtArg::Str((name.as_ptr()) as *const ::core::ffi::c_char)],
    )
        },
    }
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_parse_name(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    arname_p: *mut *mut ::core::ffi::c_char,
    memname_p: *mut *mut ::core::ffi::c_char,
) {
    let mut p: *mut ::core::ffi::c_char;
    *arname_p = xstrdup(name);
    p = strchr(*arname_p, '(' as i32);
    if p.is_null() {
        fatal(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        strlen(*arname_p) as size_t,
        b"INTERNAL: ar_parse_name: bad name '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((*arname_p) as *const ::core::ffi::c_char)],
    );
    }
    let fresh0 = p;
    p = p.offset(1_i32 as isize);
    *fresh0 = 0;
    *p.offset(strlen(p).wrapping_sub(1) as isize) = 0;
    *memname_p = p;
}
// The argument list is the fixed ar_scan callback protocol.
#[allow(clippy::too_many_arguments)]
unsafe fn ar_member_date_1(
    mut _desc: i32,
    mem: *const ::core::ffi::c_char,
    truncated: i32,
    mut _hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    date: intmax_t,
    mut _uid: i32,
    mut _gid: i32,
    mut _mode: ::core::ffi::c_uint,
    name: *const ::core::ffi::c_void,
) -> intmax_t {
    if ar_name_equal(
        ::core::ffi::CStr::from_ptr(name as *const ::core::ffi::c_char),
        ::core::ffi::CStr::from_ptr(mem),
        truncated != 0,
    ) {
        date
    } else {
        0 as intmax_t
    }
}
/// Owns a parsed `archive(member)` name split into two NUL-terminated C
/// strings inside a single buffer.
///
/// `ar_parse_name` historically did `xstrdup(name)`, overwrote the `(` and the
/// trailing `)` with NULs in place, and handed the caller back two interior
/// pointers it then had to `free`. `ParsedArName` replaces that `xstrdup`/`free`
/// ownership pair with an owned `Vec<u8>` that drops automatically: it holds
/// `archive\0member\0`, so [`arname`](Self::arname) is the leading C string and
/// [`memname`](Self::memname) starts at `member_off`. Nothing escapes — the
/// buffer is allocated, used, and dropped entirely within the calling function.
struct ParsedArName {
    /// The dup of `name` with `(` and the trailing `)` rewritten as NULs,
    /// i.e. `archive\0member\0`, kept owned so it drops at end of scope.
    buf: Vec<u8>,
    /// Byte offset of the member C string within `buf`.
    member_off: usize,
}

impl ParsedArName {
    /// Parse `name` (a well-formed `archive(member)` reference, as guaranteed by
    /// a prior [`ar_name`] check) into an owned buffer. Mirrors `ar_parse_name`:
    /// split at the first `(`, then drop the trailing `)`.
    ///
    /// Calls [`out_of_memory`] on allocation failure, matching the original
    /// `xstrdup`.
    fn parse(name: &::core::ffi::CStr) -> Self {
        let src = name.to_bytes_with_nul();
        // Reserve fallibly so OOM routes through make's `out_of_memory()`
        // ("virtual memory exhausted") diagnostic, matching the original
        // `xstrdup`, rather than aborting via Rust's allocation-error path.
        let mut buf = Vec::new();
        if buf.try_reserve_exact(src.len()).is_err() {
            out_of_memory();
        }
        buf.extend_from_slice(src);
        // `ar_name` guarantees a '(' exists and the name ends with ')'.
        let lp = buf
            .iter()
            .position(|&c| c == b'(')
            .expect("ParsedArName::parse: ar_name guarantees a '('");
        buf[lp] = 0;
        let member_off = lp + 1;
        // Overwrite the trailing ')' (the byte before the original NUL) with a
        // NUL, exactly as `p[strlen(p) - 1] = '\0'` did.
        let close = buf.len() - 2;
        buf[close] = 0;
        ParsedArName { buf, member_off }
    }

    /// The archive C string (`archive`).
    fn arname(&self) -> *const ::core::ffi::c_char {
        self.buf.as_ptr() as *const ::core::ffi::c_char
    }

    /// The member C string (`member`).
    fn memname(&self) -> *const ::core::ffi::c_char {
        self.buf[self.member_off..].as_ptr() as *const ::core::ffi::c_char
    }
}

/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_member_date(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
) -> time_t {
    // `name` is `archive(member)`; own the split buffer here so it drops on
    // return (replacing the old `ar_parse_name` xstrdup + `free`).
    let parsed = ParsedArName::parse(::core::ffi::CStr::from_ptr(name));
    let arname = parsed.arname();
    let memname = parsed.memname();
    let val: intmax_t;
    let arname_bytes = ::core::ffi::CStr::from_ptr(arname).to_bytes();
    let mut arfile = lookup_file(ctx, arname_bytes);
    if arfile.is_none() && file_exists_p(ctx, arname) != 0 {
        arfile = Some(enter_file(ctx, arname_bytes));
    }
    if let Some(fid) = arfile {
        f_mtime(ctx, fid, false);
    }
    val = ar_scan(
        ctx,
        arname,
        Some(ar_member_date_1),
        memname as *const ::core::ffi::c_void,
    );
    if (0 as intmax_t) < val
        && val
            <= (if (0_i32 as time_t) < -1_i32 as time_t {
                -1_i32 as time_t
            } else {
                (((1_i32 as time_t)
                    << (::core::mem::size_of::<time_t>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(2_usize))
                    - 1 as time_t)
                    * 2 as time_t
                    + 1 as time_t
            }) as intmax_t
    {
        val as time_t
    } else {
        -1_i32 as time_t
    }
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_touch(ctx: &crate::execctx::ExecContext, name: *const ::core::ffi::c_char) -> i32 {
    let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: i32;
    ar_parse_name(ctx, name, &raw mut arname, &raw mut memname);
    let arfile = enter_file(ctx, ::core::ffi::CStr::from_ptr(arname).to_bytes());
    f_mtime(ctx, arfile, false);
    val = 1;
    match ar_member_touch(ctx, arname, memname) {
        -1 => {
            error(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        strlen(arname) as size_t,
        b"touch: archive '%s' does not exist\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((arname) as *const ::core::ffi::c_char)],
    );
        }
        -2 => {
            error(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        strlen(arname) as size_t,
        b"touch: '%s' is not a valid archive\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((arname) as *const ::core::ffi::c_char)],
    );
        }
        -3 => {
            perror_with_name(
                ctx,
                b"touch: \0" as *const u8 as *const ::core::ffi::c_char,
                arname,
            );
        }
        1 => {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                (strlen(memname) as size_t).wrapping_add(strlen(arname) as size_t),
                b"touch: member '%s' does not exist in '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str((memname) as *const ::core::ffi::c_char),
                    FmtArg::Str((arname) as *const ::core::ffi::c_char),
                ],
            );
        }
        0 => {
            val = 0;
        }
        _ => {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                strlen(name) as size_t,
                b"touch: bad return code from ar_member_touch on '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Str((name) as *const ::core::ffi::c_char)],
            );
        }
    }
    free(arname as *mut ::core::ffi::c_void);
    val
}
// The argument list is the fixed ar_scan callback protocol.
#[allow(clippy::too_many_arguments)]
unsafe fn ar_glob_match<T: SeqNode>(
    mut _desc: i32,
    mem: *const ::core::ffi::c_char,
    mut _truncated: i32,
    mut _hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    mut _date: intmax_t,
    mut _uid: i32,
    mut _gid: i32,
    mut _mode: ::core::ffi::c_uint,
    arg: *const ::core::ffi::c_void,
) -> intmax_t {
    let state: *mut ArGlobState<T> = arg as *mut ArGlobState<T>;
    // SAFETY: set by `ar_glob` from its borrowed context; live for the scan.
    let ctx = &*(*state).ctx;
    if fnmatch((*state).pattern, mem, FNM_PATHNAME | FNM_PERIOD) == 0 {
        let new: *mut T = T::alloc();
        T::set_name(
            new,
            strcache_add(ctx, concat(&[
                (*state).arname,
                b"(\0" as *const u8 as *const ::core::ffi::c_char,
                mem,
                b")\0" as *const u8 as *const ::core::ffi::c_char,
            ])),
        );
        T::set_next(new, (*state).chain);
        (*state).chain = new;
        (*state).n = (*state).n.wrapping_add(1);
    }
    0 as intmax_t
}
/// Does `pattern` contain shell glob metacharacters (`?`, `*`, or a balanced
/// `[`…`]`)? When `quote` is set, a backslash escapes the following byte, so it
/// is skipped rather than treated as a metacharacter. Pure mirror of make's
/// `ar_glob_pattern_p`.
fn ar_glob_pattern_p(pattern: &[u8], quote: bool) -> bool {
    let mut opened = false;
    let mut i = 0;
    while i < pattern.len() {
        match pattern[i] {
            b'?' | b'*' => return true,
            // Skip the escaped byte; bounded indexing avoids the C version's
            // read past the NUL when a trailing backslash is escaped.
            b'\\' if quote => i += 1,
            b'[' => opened = true,
            b']' if opened => return true,
            _ => {}
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod ar_glob_pattern_p_tests {
    use super::ar_glob_pattern_p;

    #[test]
    fn detects_glob_metacharacters() {
        assert!(ar_glob_pattern_p(b"*.o", true));
        assert!(ar_glob_pattern_p(b"foo?", true));
        assert!(ar_glob_pattern_p(b"foo[abc].o", true));
    }

    #[test]
    fn plain_names_are_not_patterns() {
        assert!(!ar_glob_pattern_p(b"", true));
        assert!(!ar_glob_pattern_p(b"foo.o", true));
        // A ']' without a preceding '[' is not a class.
        assert!(!ar_glob_pattern_p(b"foo].o", true));
    }

    #[test]
    fn backslash_quoting() {
        // With quoting on, the escaped '*' is consumed, not matched.
        assert!(!ar_glob_pattern_p(b"foo\\*", true));
        // With quoting off, the backslash is inert and the '*' still matches.
        assert!(ar_glob_pattern_p(b"foo\\*", false));
        // A trailing backslash must not read past the end of the slice.
        assert!(!ar_glob_pattern_p(b"foo\\", true));
    }
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_glob<T: SeqNode>(
    ctx: &crate::execctx::ExecContext,
    arname: *const ::core::ffi::c_char,
    member_pattern: *const ::core::ffi::c_char,
) -> *mut T {
    let mut state: ArGlobState<T> = ArGlobState {
        ctx,
        arname: ::core::ptr::null::<::core::ffi::c_char>(),
        pattern: ::core::ptr::null::<::core::ffi::c_char>(),
        chain: ::core::ptr::null_mut::<T>(),
        n: 0,
    };
    if !ar_glob_pattern_p(::core::ffi::CStr::from_ptr(member_pattern).to_bytes(), true) {
        return ::core::ptr::null_mut::<T>();
    }
    state.arname = arname;
    state.pattern = member_pattern;
    state.chain = ::core::ptr::null_mut::<T>();
    state.n = 0;
    ar_scan(
        ctx,
        arname,
        Some(ar_glob_match::<T>),
        &raw mut state as *const ::core::ffi::c_void,
    );
    if state.chain.is_null() {
        return ::core::ptr::null_mut::<T>();
    }
    // Gather the member-name pointers from the chain, sort them with make's
    // `alpha_compare` ordering (now the safe `alpha_cmp` over the C strings'
    // bytes), then write the sorted names back into the chain in order. This
    // replaces a hand-rolled `*mut *const c_char` scratch buffer and a libc
    // `qsort` with an idiomatic `Vec` + `slice::sort_by`.
    let mut names: Vec<*const ::core::ffi::c_char> = Vec::with_capacity(state.n as usize);
    let mut n = state.chain;
    while !n.is_null() {
        names.push(T::name(n));
        n = T::next(n);
    }
    names.sort_by(|&a, &b| {
        alpha_cmp(
            ::core::ffi::CStr::from_ptr(a).to_bytes(),
            ::core::ffi::CStr::from_ptr(b).to_bytes(),
        )
    });
    let mut n = state.chain;
    for &name in &names {
        T::set_name(n, name);
        n = T::next(n);
    }
    state.chain
}
pub const __CHAR_BIT__: i32 = 8;

#[cfg(test)]
mod ar_name_tests {
    use super::{classify_ar_name, ArName};

    fn kind(s: &str) -> &'static str {
        match classify_ar_name(s.as_bytes()) {
            ArName::Plain => "plain",
            ArName::Member => "member",
            ArName::Unsupported => "unsupported",
        }
    }

    #[test]
    fn plain_names_are_not_archives() {
        assert_eq!(kind("foo.o"), "plain"); // no parenthesis
        assert_eq!(kind(""), "plain"); // empty
        assert_eq!(kind("(member)"), "plain"); // '(' at the very start
        assert_eq!(kind("lib("), "plain"); // no closing ')'
        assert_eq!(kind("lib(member"), "plain"); // missing ')'
        assert_eq!(kind("lib()"), "plain"); // empty member
        assert_eq!(kind("lib(member)x"), "plain"); // ')' is not the last byte
    }

    #[test]
    fn well_formed_archive_members() {
        assert_eq!(kind("lib.a(member.o)"), "member");
        assert_eq!(kind("lib(m)"), "member");
        assert_eq!(kind("a(bc)"), "member");
    }

    #[test]
    fn nested_form_is_unsupported() {
        assert_eq!(kind("lib((member))"), "unsupported");
        assert_eq!(kind("a((b))"), "unsupported");
    }

    #[test]
    fn single_inner_open_paren_is_a_member() {
        // Only the unsupported form needs both an inner '(' and a matching
        // inner ')'. A lone inner '(' is treated as part of the member name.
        assert_eq!(kind("lib((member)"), "member");
    }
}

#[cfg(test)]
mod parsed_ar_name_tests {
    use super::ParsedArName;

    /// Pre-conversion oracle: the in-place split that `ar_parse_name` did on an
    /// `xstrdup`'d copy. Operates on an owned `Vec<u8>` (mirroring the dup),
    /// overwrites the first `(` and the trailing `)` with NULs, and returns the
    /// resulting `(arname, memname)` C-string byte slices (without their NULs),
    /// reproducing the interior pointers `ar_parse_name` handed back.
    fn ar_parse_name_oracle(name: &[u8]) -> (Vec<u8>, Vec<u8>) {
        // xstrdup: a NUL-terminated copy.
        let mut buf = name.to_vec();
        buf.push(0);
        // p = strchr(arname, '('); *(p++) = '\0';
        let lp = buf
            .iter()
            .position(|&c| c == b'(')
            .expect("oracle: name must contain '('");
        buf[lp] = 0;
        // p[strlen(p) - 1] = '\0';  (the trailing ')' before the copy's NUL).
        let close = buf.len() - 2;
        buf[close] = 0;
        let arname = buf[..lp].to_vec();
        let memname = buf[lp + 1..close].to_vec();
        (arname, memname)
    }

    /// Drive the safe `ParsedArName` and the preserved oracle through the same
    /// inputs and assert byte-identical archive/member strings, including
    /// embedded NUL-adjacent, high-byte, and single-character members.
    fn assert_same(name: &[u8]) {
        let cs = ::std::ffi::CString::new(name).expect("test input has no embedded NUL");
        let parsed = ParsedArName::parse(&cs);
        // Read the produced C strings back as byte slices (no terminating NUL).
        let got_ar = unsafe { ::core::ffi::CStr::from_ptr(parsed.arname()) }.to_bytes();
        let got_mem = unsafe { ::core::ffi::CStr::from_ptr(parsed.memname()) }.to_bytes();
        let (want_ar, want_mem) = ar_parse_name_oracle(name);
        assert_eq!(got_ar, &want_ar[..], "arname mismatch for {name:?}");
        assert_eq!(got_mem, &want_mem[..], "memname mismatch for {name:?}");
    }

    #[test]
    fn matches_oracle_on_representative_inputs() {
        assert_same(b"lib.a(member.o)");
        assert_same(b"a(b)"); // single-char archive and member
        assert_same(b"lib((member)"); // lone inner '(' is part of the member
        assert_same(b"/path/to/lib.a(very_long_member_name.o)");
        assert_same(b"\xc3\xa9lib.a(m\xc3\xa9mber.o)"); // high bytes both sides
        assert_same(b"x(\xff)"); // high-byte single-byte member
    }
}
